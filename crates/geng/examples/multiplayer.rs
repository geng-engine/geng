use geng::{prelude::*, TextAlign};
use geng_net_simple as simple_net;

type PlayerId = usize;

#[derive(Debug, Serialize, Deserialize, Diff, Clone, PartialEq)]
struct Player {
    id: PlayerId,
    position: vec2<f32>,
}

impl HasId for Player {
    type Id = PlayerId;
    fn id(&self) -> &PlayerId {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize, Diff, Clone, PartialEq)]
struct Model {
    current_time: f32,
    next_player_id: PlayerId,
    players: Collection<Player>,
}

impl Model {
    fn new() -> Self {
        Self {
            current_time: 0.0,
            next_player_id: 1,
            players: Collection::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    UpdatePosition(vec2<f32>),
}

impl simple_net::Model for Model {
    type PlayerId = PlayerId;
    type Message = Message;
    type Event = ();
    const TICKS_PER_SECOND: f32 = 20.0;
    fn new_player(&mut self, _events: &mut Vec<()>) -> Self::PlayerId {
        let player_id = self.next_player_id;
        self.next_player_id += 1;
        self.players.insert(Player {
            id: player_id,
            position: vec2(
                thread_rng().gen_range(-5.0..=5.0),
                thread_rng().gen_range(-5.0..=5.0),
            ),
        });
        player_id
    }
    fn drop_player(&mut self, _events: &mut Vec<()>, player_id: &PlayerId) {
        self.players.remove(player_id);
    }
    fn handle_message(
        &mut self,
        _events: &mut Vec<()>,
        player_id: &PlayerId,
        message: Self::Message,
    ) -> Vec<Self::Event> {
        match message {
            Message::UpdatePosition(position) => {
                self.players.get_mut(player_id).unwrap().position = position;
            }
        }
        vec![]
    }
    fn tick(&mut self, _events: &mut Vec<()>) {
        self.current_time += 1.0 / Self::TICKS_PER_SECOND;
    }
}

struct Game {
    geng: Geng,
    traffic_watcher: geng::net::TrafficWatcher,
    next_update: f32,
    player: Player,
    model: simple_net::Remote<Model>,
    current_time: f32,
}

impl Game {
    fn new(geng: &Geng, player_id: PlayerId, model: simple_net::Remote<Model>) -> Self {
        let current_time = model.get().current_time;
        let player = model.get().players.get(&player_id).unwrap().clone();
        Self {
            geng: geng.clone(),
            traffic_watcher: geng::net::TrafficWatcher::new(),
            next_update: 0.0,
            model,
            player,
            current_time,
        }
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {
        self.model.update();
        self.traffic_watcher.update(&self.model.traffic());
        let delta_time = delta_time as f32;

        self.current_time += delta_time;

        const SPEED: f32 = 10.0;
        if self.geng.window().is_key_pressed(geng::Key::Left)
            || self.geng.window().is_key_pressed(geng::Key::A)
        {
            self.player.position.x -= SPEED * delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::Right)
            || self.geng.window().is_key_pressed(geng::Key::D)
        {
            self.player.position.x += SPEED * delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::Up)
            || self.geng.window().is_key_pressed(geng::Key::W)
        {
            self.player.position.y += SPEED * delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::Down)
            || self.geng.window().is_key_pressed(geng::Key::S)
        {
            self.player.position.y -= SPEED * delta_time;
        }

        self.next_update -= delta_time;
        if self.next_update < 0.0 {
            while self.next_update < 0.0 {
                self.next_update += 1.0 / <Model as simple_net::Model>::TICKS_PER_SECOND;
            }
            self.model
                .send(Message::UpdatePosition(self.player.position));
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let camera = geng::Camera2d {
            center: vec2(0.0, 0.0),
            rotation: 0.0,
            fov: 100.0,
        };
        let model = self.model.get();
        for player in &model.players {
            self.geng
                .draw2d()
                .circle(framebuffer, &camera, player.position, 1.0, Rgba::GRAY);
        }
        self.geng
            .draw2d()
            .circle(framebuffer, &camera, self.player.position, 1.0, Rgba::WHITE);
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            &format!("Server time: {:.1}", model.current_time),
            vec2::splat(TextAlign::LEFT),
            mat3::translate(vec2(0.0, 0.0)) * mat3::scale_uniform(32.0),
            Rgba::WHITE,
        );
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            &format!("Client time: {:.1}", self.current_time),
            vec2::splat(TextAlign::LEFT),
            mat3::translate(vec2(0.0, 32.0)) * mat3::scale_uniform(32.0),
            Rgba::WHITE,
        );
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            &format!("traffic: {}", self.traffic_watcher),
            vec2::splat(TextAlign::LEFT),
            mat3::translate(vec2(0.0, 32.0 * 2.0)) * mat3::scale_uniform(32.0),
            Rgba::WHITE,
        );
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    simple_net::run("Multiplayer", Model::new, Game::new);
}
