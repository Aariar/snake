use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use rand::prelude::random;
use std::time::Duration;

const WIDTH: u32 = 50; // 横マス数
const HEIGHT: u32 = 50; // 縦マス数
const SPEED: u64 = 100; // 蛇速度
const POP: u64 = 500; // 出現頻度
const TAIL: bool = false; // 自動尾縮
const WINWID: u32 = 1000; // Window 幅
const WINHGT: u32 = 1000; // Window 高さ

struct HeadMaterial(Handle<ColorMaterial>); // 頭
struct SegmentMaterial(Handle<ColorMaterial>); // 尾
struct SnakeHead {
    next_segment: Entity,
}
#[derive(Default)]
struct SnakeSegment {
    next_segment: Option<Entity>,
}
struct SnakeMoveTimer(Timer);

struct FoodSpawnTimer(Timer);
struct FoodMaterial(Handle<ColorMaterial>);
struct Food;

struct GameOverEvent;

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(HeadMaterial(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));
    commands.insert_resource(SegmentMaterial(
        materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    ));
    commands.insert_resource(FoodMaterial(
        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    ));
}

fn game_setup(
    commands: Commands,
    head_material: Res<HeadMaterial>,
    segment_material: Res<SegmentMaterial>,
) {
    spawn_initial_snake(commands, head_material.0, segment_material.0);
}

fn spawn_segment(commands: &mut Commands, material: Handle<ColorMaterial>, position: Position) {
    commands
        .spawn(SpriteComponents {
            material,
            ..Default::default()
        })
        .with(SnakeSegment { next_segment: None })
        .with(position)
        .with(Size::square(0.65));
}

fn spawn_initial_snake(
    mut commands: Commands,
    head_material: Handle<ColorMaterial>,
    segment_material: Handle<ColorMaterial>,
) {
    spawn_segment(&mut commands, segment_material, Position { x: 10, y: 10 });
    let first_segment = commands.current_entity().unwrap();
    commands
        .spawn(SpriteComponents {
            material: head_material,
            ..Default::default()
        })
        .with(SnakeHead {
            next_segment: first_segment,
        })
        .with(Position { x: 10, y: 10 })
        .with(Size::square(0.8));
}

fn snake_movement(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_timer: ResMut<SnakeMoveTimer>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
    segment_material: Res<SegmentMaterial>,
    mut head_positions: Query<(&mut SnakeHead, &mut Position)>,
    segments: Query<&mut SnakeSegment>,
    positions: Query<&mut Position>,
    mut food_positions: Query<(Entity, &Food, &Position)>,
) {
    snake_timer.0.tick(time.delta_seconds);
    for (head, mut head_pos) in &mut head_positions.iter() {
        if snake_timer.0.finished {
            let mut last_position = *head_pos;
            let mut segment_entity = head.next_segment;
            if head_pos.x < 0 { head_pos.x += 2 } // 画面外 防止
            if head_pos.y < 0 { head_pos.y += 2 }
            if head_pos.x as u32 > WIDTH { head_pos.x -= 2 }
            if head_pos.y as u32 > HEIGHT { head_pos.y -= 2 }
            let mut change = false;
            if keyboard_input.pressed(KeyCode::Left) {
                head_pos.x -= 1; change = true;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                head_pos.y -= 1; change = true;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                head_pos.y += 1; change = true;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                head_pos.x += 1; change = true;
            }
            if keyboard_input.pressed(KeyCode::Return) {
                game_over_events.send(GameOverEvent);
            }
            if TAIL || change { // 尾縮
                loop {
                    let segment = segments.get::<SnakeSegment>(segment_entity).unwrap();
                    let mut segment_position = positions.get_mut::<Position>(segment_entity).unwrap();
                    std::mem::swap(&mut last_position, &mut *segment_position);
                    if let Some(n) = segment.next_segment {
                    segment_entity = n;
                    } else {
                    break;
                    }
                }
            }
            for (ent, _food, food_pos) in &mut food_positions.iter() {
                if food_pos == &*head_pos {
                    spawn_segment(&mut commands, segment_material.0, last_position);
                    let new_segment = commands.current_entity();
                    let mut segment = segments.get_mut::<SnakeSegment>(segment_entity).unwrap();
                    segment.next_segment = new_segment;
                    commands.despawn(ent);
                }
            }
        }
    }
}

fn game_over_system(
    mut commands: Commands,
    mut reader: Local<EventReader<GameOverEvent>>,
    game_over_events: Res<Events<GameOverEvent>>,
    segment_material: Res<SegmentMaterial>,
    head_material: Res<HeadMaterial>,
    mut segments: Query<(Entity, &SnakeSegment)>,
    mut food: Query<(Entity, &Food)>,
    mut heads: Query<(Entity, &SnakeHead)>,
) {
    if reader.iter(&game_over_events).next().is_some() {
        for (ent, _segment) in &mut segments.iter() {
            commands.despawn(ent);
        }
        for (ent, _food) in &mut food.iter() {
            commands.despawn(ent);
        }
        for (ent, _head) in &mut heads.iter() {
            commands.despawn(ent);
        }
        spawn_initial_snake(commands, head_material.0, segment_material.0);
    }
}

fn food_spawner(
    mut commands: Commands,
    food_material: Res<FoodMaterial>,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: food_material.0,
                ..Default::default()
            })
            .with(Food)
            .with(Position {
                x: (random::<f32>() * WIDTH as f32) as i32,
                y: (random::<f32>() * HEIGHT as f32) as i32,
            })
            .with(Size::square(0.8));
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    for (size, mut sprite) in &mut q.iter() {
        let window = windows.get_primary().unwrap();
        sprite.size = Vec2::new(
            size.width as f32 / WIDTH as f32 * window.width as f32,
            size.height as f32 / HEIGHT as f32 * window.height as f32,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(p: f32, bound_window: f32, bound_game: f32) -> f32 {
        p / bound_game * bound_window - (bound_window / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in &mut q.iter() {
        transform.set_translation(Vec3::new(
            convert(pos.x as f32, window.width as f32, WIDTH as f32),
            convert(pos.y as f32, window.height as f32, HEIGHT as f32),
            0.0,
        ))
    }
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(WindowDescriptor {
            title: "Snake - AariaToys".to_string(),
            width: WINWID,
            height: WINHGT,
            ..Default::default()
        })
        .add_resource(SnakeMoveTimer(Timer::new(
            Duration::from_millis(SPEED),
            true,
        )))
        .add_event::<GameOverEvent>()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_resource(FoodSpawnTimer(Timer::new(
            Duration::from_millis(POP),
            true,
        )))
        .add_system(food_spawner.system())
        .add_system(game_over_system.system())
        .add_default_plugins()
        .run();
}
