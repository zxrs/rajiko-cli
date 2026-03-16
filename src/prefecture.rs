use std::fmt::Display;

pub enum Area {
    HokkaidoTohoku([Pref; 7]),
    Kanto([Pref; 7]),
    HokurikuKoshinetsu([Pref; 6]),
    Chubu([Pref; 4]),
    Kinki([Pref; 6]),
    ChugokuShikoku([Pref; 9]),
    KyusyuOkinawa([Pref; 8]),
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Area::*;
        match self {
            HokkaidoTohoku(_) => f.write_str("北海道・東北"),
            Kanto(_) => f.write_str("関東"),
            HokurikuKoshinetsu(_) => f.write_str("北陸・甲信越"),
            Chubu(_) => f.write_str("中部"),
            Kinki(_) => f.write_str("近畿"),
            ChugokuShikoku(_) => f.write_str("中国・四国"),
            KyusyuOkinawa(_) => f.write_str("九州・沖縄"),
        }
    }
}

const AREA: [Area; 7] = [
    Area::HokkaidoTohoku([
        Pref::Hokkaido,
        Pref::Aomori,
        Pref::Iwate,
        Pref::Miyagi,
        Pref::Akita,
        Pref::Yamagata,
        Pref::Fukushima,
    ]),
    Area::Kanto([
        Pref::Ibaraki,
        Pref::Tochigi,
        Pref::Gunma,
        Pref::Saitama,
        Pref::Chiba,
        Pref::Tokyo,
        Pref::Kanagawa,
    ]),
    Area::HokurikuKoshinetsu([
        Pref::Niigata,
        Pref::Yamanashi,
        Pref::Nagano,
        Pref::Ishikawa,
        Pref::Toyama,
        Pref::Fukui,
    ]),
    Area::Chubu([Pref::Aichi, Pref::Gifu, Pref::Shizuoka, Pref::Mie]),
    Area::Kinki([
        Pref::Osaka,
        Pref::Hyogo,
        Pref::Kyoto,
        Pref::Shiga,
        Pref::Nara,
        Pref::Wakayama,
    ]),
    Area::ChugokuShikoku([
        Pref::Okayama,
        Pref::Hiroshima,
        Pref::Tottori,
        Pref::Shimane,
        Pref::Yamaguchi,
        Pref::Kagawa,
        Pref::Tokushima,
        Pref::Ehime,
        Pref::Kochi,
    ]),
    Area::KyusyuOkinawa([
        Pref::Fukuoka,
        Pref::Saga,
        Pref::Nagasaki,
        Pref::Kumamoto,
        Pref::Oita,
        Pref::Miyazaki,
        Pref::Kagoshima,
        Pref::Okinawa,
    ]),
];

#[derive(Debug, Clone, Copy)]
pub enum Pref {
    // Hokkaido_Tohoku
    Hokkaido,
    Aomori,
    Iwate,
    Miyagi,
    Akita,
    Yamagata,
    Fukushima,

    // Kanto
    Ibaraki,
    Tochigi,
    Gunma,
    Saitama,
    Chiba,
    Tokyo,
    Kanagawa,

    // HokurikuKoshinetsu
    Niigata,
    Yamanashi,
    Nagano,
    Ishikawa,
    Toyama,
    Fukui,

    // Chubu
    Aichi,
    Gifu,
    Shizuoka,
    Mie,

    // Kinki
    Osaka,
    Hyogo,
    Kyoto,
    Shiga,
    Nara,
    Wakayama,

    // Chugoku_Shikoku
    Okayama,
    Hiroshima,
    Tottori,
    Shimane,
    Yamaguchi,
    Kagawa,
    Tokushima,
    Ehime,
    Kochi,

    // Kyusyu_Okinawa
    Fukuoka,
    Saga,
    Nagasaki,
    Kumamoto,
    Oita,
    Miyazaki,
    Kagoshima,
    Okinawa,
}
