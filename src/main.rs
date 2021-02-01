// Rust APIガイドライン (非公式日本語訳)
// https://sinkuu.github.io/api-guidelines/about.html

extern crate bmp;
/*
    bRustには2種類の定数があり、いずれもグローバルスコープを含む任意のスコープで宣言することができます。また、いずれも型を明示しなくてはなりません。

    const: 不変の値（通常はこちらを使用する）
    static: スタティックなライフタイムを持つミュータブル(mut)な値 The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
*/
const FIELD_WIDTH: usize = 136;
const FIELD_HEIGHT: usize = 136;

enum MapChip {
    SEA,
    SEA2,
    PLANE,
    FOREST,
    MOUNTAIN,
    ROCK,
    WALL,
    DESERT,
    POISON,
    BRIDGE,
    CASTLE,
    TOWN,
    DUNGEON,
    SHIRINE,
    STAIRS,
    MAX,
}

#[allow(dead_code)]
struct MapChipInfo {
    id: MapChip,
    color: (u8, u8, u8),
    can_enter: bool,
    symbol: char,
}

static MAP_CHIP_DICT: [MapChipInfo; MapChip::MAX as usize] = [
    // SEA
    MapChipInfo {
        id: MapChip::SEA,
        color: (91, 119, 234),
        symbol: '〜',
        can_enter: false,
    },
    // SEA 2
    MapChipInfo {
        id: MapChip::SEA2,
        color: (90, 119, 234),
        symbol: '波',
        can_enter: false,
    },
    // PLANE
    MapChipInfo {
        id: MapChip::PLANE,
        color: (121, 195, 0),
        symbol: '．',
        can_enter: true,
    },
    // FOREST
    MapChipInfo {
        id: MapChip::FOREST,
        color: (50, 132, 5),
        symbol: '木',
        can_enter: true,
    },
    // MOUNTAIN
    MapChipInfo {
        id: MapChip::MOUNTAIN,
        color: (123, 158, 23),
        symbol: '山',
        can_enter: true,
    },
    // ROCK
    MapChipInfo {
        id: MapChip::ROCK,
        color: (109, 128, 70),
        symbol: '岩',
        can_enter: false,
    },
    // WALL
    MapChipInfo {
        id: MapChip::WALL,
        color: (154, 154, 154),
        symbol: '壁',
        can_enter: false,
    },
    // DESERT
    MapChipInfo {
        id: MapChip::DESERT,
        color: (231, 209, 119),
        symbol: '砂',
        can_enter: true,
    },
    // POISON
    MapChipInfo {
        id: MapChip::POISON,
        color: (16, 45, 0),
        symbol: '毒',
        can_enter: true,
    },
    // BRIDGE
    MapChipInfo {
        id: MapChip::BRIDGE,
        color: (111, 117, 142),
        symbol: '橋',
        can_enter: true,
    },
    // CASTLE
    MapChipInfo {
        id: MapChip::CASTLE,
        color: (117, 118, 122),
        symbol: '城',
        can_enter: true,
    },
    // TOWN
    MapChipInfo {
        id: MapChip::TOWN,
        color: (126, 127, 130),
        symbol: '町',
        can_enter: true,
    },
    // DUNGEON
    MapChipInfo {
        id: MapChip::DUNGEON,
        color: (62, 75, 37),
        symbol: '穴',
        can_enter: true,
    },
    // SHRINE
    MapChipInfo {
        id: MapChip::SHIRINE,
        color: (167, 87, 167),
        symbol: '祠',
        can_enter: true,
    },
    // STAIRS
    MapChipInfo {
        id: MapChip::STAIRS,
        color: (100, 109, 81),
        symbol: '階',
        can_enter: true,
    },
];

fn main() {
    create_map();
}

fn create_map() {
    // BMPファイルの読み込み
    let img = bmp::open("/home/seiji/dev/rust/dq_rust/assets/map.bmp").unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    // 1ピクセル単位で処理
    let mut map: [[i32; FIELD_WIDTH]; FIELD_HEIGHT] = [[0; FIELD_WIDTH]; FIELD_HEIGHT];
    for (x, y) in img.coordinates() {
        let _pixel = img.get_pixel(x, y);
        let max = MapChip::MAX as usize;
        for n in 0..max {
            let map_chip_info = &MAP_CHIP_DICT[n];
            if map_chip_info.color.0 == _pixel.r
                && map_chip_info.color.1 == _pixel.g
                && map_chip_info.color.2 == _pixel.b
            {
                map[x as usize][y as usize] = n as i32;
            }
        }
    }

    // MAPの表示をテスト
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let map_chip_id = map[x][y] as usize;
            let map_chip_info = &MAP_CHIP_DICT[map_chip_id];

            print!("{}", map_chip_info.symbol);
        }
        println!("");
    }
}
