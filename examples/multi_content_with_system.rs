use bevy::prelude::*;
use bevy::reflect::TypePath;

use bevy_ninepatch::{
    NinePatchBuilder, NinePatchBundle, NinePatchContent, NinePatchData, NinePatchPlugin,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        // Add the `NinePatchPlugin` plugin
        .add_plugins(NinePatchPlugin::<Content>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, set_content)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
) {
    // prepare the panel
    let panel_texture_handle = asset_server.load("glassPanel_corners.png");
    let panel_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins_with_content(
        20,
        20,
        20,
        20,
        Content::Panel,
    ));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(500.),
                height: Val::Px(300.),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: panel_nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(Camera2dBundle::default());
}

fn set_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
    mut query: Query<(Entity, &mut NinePatchContent<Content>)>,
) {
    for (entity, mut nine_patch_content) in query.iter_mut() {
        if !nine_patch_content.loaded {
            match nine_patch_content.content {
                Content::Panel => {
                    // prepare the button
                    let button_texture_handle = asset_server.load("blue_button02.png");
                    let button_nine_patch_handle = nine_patches.add(
                        NinePatchBuilder::by_margins_with_content(5, 10, 6, 6, Content::Button),
                    );

                    let content_entity = commands
                        .spawn(
                            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
                            // of this entity
                            NinePatchBundle {
                                style: Style {
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Px(0.),
                                        top: Val::Auto,
                                        bottom: Val::Px(0.),
                                    },
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(200.),
                                    height: Val::Px(100.),
                                    ..Default::default()
                                },
                                nine_patch_data: NinePatchData {
                                    nine_patch: button_nine_patch_handle,
                                    texture: button_texture_handle,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
                Content::Button => {
                    // load font
                    let font = asset_server.load("Kenney Future Narrow.ttf");

                    let content_entity = commands
                        .spawn(
                            TextBundle::from_section(
                                "OK",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 50.0,
                                    color: Color::GREEN,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    left: Val::Px(60.),
                                    right: Val::Auto,
                                    top: Val::Px(20.),
                                    bottom: Val::Auto,
                                },
                                ..Default::default()
                            }),
                        )
                        .id();
                    commands.entity(entity).push_children(&[content_entity]);
                    nine_patch_content.loaded = true;
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, TypePath, std::hash::Hash)]
enum Content {
    Panel,
    Button,
}
