use crate::config::Player;
use crate::constant::HEROES;
use image::{GenericImageView, Pixel, Rgb, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};
use std::include_bytes;
use std::path::Path;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 72;
const MAX_HEROES_BY_PLAYER: u32 = 5;
const MARGE: u32 = 1;
const MAX_COLS: u32 = 2;
const NU_WITDH: u32 = 13;
const NU_HEIGHT: u32 = 18;
const SMALL_WITDH_MARGE: u32 = WIDTH / 10;
const WIDTH_USE: u32 = SMALL_WITDH_MARGE + WIDTH;
const SMALL_HEIGHT_MARGE: u32 = HEIGHT / 10;
const HEIGHT_USE: u32 = SMALL_HEIGHT_MARGE + HEIGHT;

fn draw_text(image: &mut RgbaImage, text: &str, x: u32, y: u32) {
    let height = 34.4;
    let scale = Scale {
        x: height,
        y: height,
    };
    let font = Vec::from(include_bytes!("../font.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    draw_text_mut(
        image,
        Rgb([153u8, 0u8, 153u8]).to_rgba(),
        x,
        y,
        scale,
        &font,
        text,
    );
}

fn draw_player(image: &mut RgbaImage, player: &Player, absolute_x: u32, absolute_y: u32) {
    let rect = Rect::at(
        (absolute_x * WIDTH_USE - SMALL_WITDH_MARGE) as i32,
        (absolute_y * HEIGHT_USE - SMALL_HEIGHT_MARGE) as i32,
    )
    .of_size(
        if player.expert.len() as u32 != 0 {
            MAX_HEROES_BY_PLAYER * WIDTH_USE + SMALL_WITDH_MARGE
        } else {
            450
        },
        (player.expert.len() as u32 / MAX_HEROES_BY_PLAYER + 1) * HEIGHT_USE + SMALL_HEIGHT_MARGE,
    );
    draw_filled_rect_mut(image, rect, Rgb([0u8, 177u8, 106u8]).to_rgba());
    for (idx, hero) in player.expert.iter().enumerate() {
        let idx = idx as u32;
        let hero_image = HEROES.get(hero).unwrap();

        for (x, y, pixel) in hero_image.pixels() {
            let new_x = (idx % MAX_HEROES_BY_PLAYER) * WIDTH_USE + absolute_x * WIDTH_USE + x;
            let new_y = (idx / MAX_HEROES_BY_PLAYER) * HEIGHT_USE + absolute_y * HEIGHT_USE + y;
            image.put_pixel(new_x, new_y, pixel);
        }
    }
    let global_idx = player.expert.len() as u32
        + (MAX_HEROES_BY_PLAYER - player.expert.len() as u32 % MAX_HEROES_BY_PLAYER);
    let rect = Rect::at(
        (absolute_x * WIDTH_USE - SMALL_WITDH_MARGE) as i32,
        ((absolute_y + player.expert.len() as u32 / MAX_HEROES_BY_PLAYER + 1) * HEIGHT_USE
            - SMALL_HEIGHT_MARGE) as i32,
    )
    .of_size(
        if player.op.is_empty() {
            1
        } else {
            MAX_HEROES_BY_PLAYER * WIDTH_USE + SMALL_WITDH_MARGE
        },
        (player.op.len() as u32 / MAX_HEROES_BY_PLAYER + 1) * HEIGHT_USE + SMALL_HEIGHT_MARGE,
    );
    if !player.op.is_empty() {
        draw_filled_rect_mut(image, rect, Rgb([230u8, 126u8, 34u8]).to_rgba());
    }

    for (idx, hero) in player.op.iter().enumerate() {
        let idx = global_idx + idx as u32;
        let hero_image = HEROES.get(hero).unwrap();

        for (x, y, pixel) in hero_image.pixels() {
            let new_x = (idx % MAX_HEROES_BY_PLAYER) * WIDTH_USE + absolute_x * WIDTH_USE + x;
            let new_y = (idx / MAX_HEROES_BY_PLAYER) * HEIGHT_USE + absolute_y * HEIGHT_USE + y;
            image.put_pixel(new_x, new_y, pixel);
        }
    }
}

pub fn create_image_heroes(players: &Vec<Player>) {
    let path = Path::new("./image.png");
    let mut image = RgbaImage::new(WIDTH_USE * NU_WITDH, HEIGHT_USE * NU_HEIGHT);
    for p in image.pixels_mut() {
        p.0 = [47u8, 50u8, 52u8, 255u8];
    }
    for (idx, player) in players.iter().enumerate() {
        let idx = idx as u32;
        let x = MARGE + (idx % MAX_COLS * MAX_HEROES_BY_PLAYER) + idx % MAX_COLS;
        let y = MARGE + (idx / MAX_COLS * MAX_HEROES_BY_PLAYER);
        draw_text(
            &mut image,
            player.name.to_uppercase().as_str(),
            x * WIDTH_USE,
            y * HEIGHT_USE + HEIGHT/3,
        );
        draw_player(&mut image, player, x, y + 1);
    }
    let _ = image.save(path).unwrap();
}
