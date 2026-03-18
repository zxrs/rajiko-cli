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

impl Display for Pref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Pref::*;
        match self {
            Hokkaido => f.write_str("北海道"),
            Aomori => f.write_str("青森"),
            Iwate => f.write_str("岩手"),
            Miyagi => f.write_str("宮城"),
            Akita => f.write_str("秋田"),
            Yamagata => f.write_str("山形"),
            Fukushima => f.write_str("福島"),
            Ibaraki => f.write_str("茨城"),
            Tochigi => f.write_str("栃木"),
            Gunma => f.write_str("群馬"),
            Saitama => f.write_str("埼玉"),
            Chiba => f.write_str("千葉"),
            Tokyo => f.write_str("東京"),
            Kanagawa => f.write_str("神奈川"),
            Niigata => f.write_str("新潟"),
            Yamanashi => f.write_str("山梨"),
            Nagano => f.write_str("長野"),
            Ishikawa => f.write_str("石川"),
            Toyama => f.write_str("富山"),
            Fukui => f.write_str("福井"),
            Aichi => f.write_str("愛知"),
            Gifu => f.write_str("岐阜"),
            Shizuoka => f.write_str("静岡"),
            Mie => f.write_str("三重"),
            Osaka => f.write_str("大阪"),
            Hyogo => f.write_str("兵庫"),
            Kyoto => f.write_str("京都"),
            Shiga => f.write_str("滋賀"),
            Nara => f.write_str("奈良"),
            Wakayama => f.write_str("和歌山"),
            Okayama => f.write_str("岡山"),
            Hiroshima => f.write_str("広島"),
            Tottori => f.write_str("鳥取"),
            Shimane => f.write_str("島根"),
            Yamaguchi => f.write_str("山口"),
            Kagawa => f.write_str("香川"),
            Tokushima => f.write_str("徳島"),
            Ehime => f.write_str("愛媛"),
            Kochi => f.write_str("高知"),
            Fukuoka => f.write_str("福岡"),
            Saga => f.write_str("佐賀"),
            Nagasaki => f.write_str("長崎"),
            Kumamoto => f.write_str("熊本"),
            Oita => f.write_str("大分"),
            Miyazaki => f.write_str("宮崎"),
            Kagoshima => f.write_str("鹿児島"),
            Okinawa => f.write_str("沖縄"),
        }
    }
}

impl Pref {
    pub const fn location(&self) -> (f32, f32) {
        use Pref::*;
        match self {
            Hokkaido => (43.064615, 141.346807),
            Aomori => (40.824308, 140.739998),
            Iwate => (39.703619, 141.152684),
            Miyagi => (38.268837, 140.8721),
            Akita => (39.718614, 140.102364),
            Yamagata => (38.240436, 140.363633),
            Fukushima => (37.750299, 140.467551),
            Ibaraki => (36.341811, 140.446793),
            Tochigi => (36.565725, 139.883565),
            Gunma => (36.390668, 139.060406),
            Saitama => (35.856999, 139.648849),
            Chiba => (35.605057, 140.123306),
            Tokyo => (35.689488, 139.691706),
            Kanagawa => (35.447507, 139.642345),
            Niigata => (37.902552, 139.023095),
            Toyama => (36.695291, 137.211338),
            Ishikawa => (36.594682, 136.625573),
            Fukui => (36.065178, 136.221527),
            Yamanashi => (35.664158, 138.568449),
            Nagano => (36.651299, 138.180956),
            Gifu => (35.391227, 136.722291),
            Shizuoka => (34.97712, 138.383084),
            Aichi => (35.180188, 136.906565),
            Mie => (34.730283, 136.508588),
            Shiga => (35.004531, 135.86859),
            Kyoto => (35.021247, 135.755597),
            Osaka => (34.686297, 135.519661),
            Hyogo => (34.691269, 135.183071),
            Nara => (34.685334, 135.832742),
            Wakayama => (34.225987, 135.167509),
            Tottori => (35.503891, 134.237736),
            Shimane => (35.472295, 133.0505),
            Okayama => (34.661751, 133.934406),
            Hiroshima => (34.39656, 132.459622),
            Yamaguchi => (34.185956, 131.470649),
            Tokushima => (34.065718, 134.55936),
            Kagawa => (34.340149, 134.043444),
            Ehime => (33.841624, 132.765681),
            Kochi => (33.559706, 133.531079),
            Fukuoka => (33.606576, 130.418297),
            Saga => (33.249442, 130.299794),
            Nagasaki => (32.744839, 129.873756),
            Kumamoto => (32.789827, 130.741667),
            Oita => (33.238172, 131.612619),
            Miyazaki => (31.911096, 131.423893),
            Kagoshima => (31.560146, 130.557978),
            Okinawa => (26.2124, 127.680932),
        }
    }
}
