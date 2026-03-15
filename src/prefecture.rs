use std::fmt::Display;

pub enum Area {
    HokkaidoTohoku(HokkaidoTohoku),
    Kanto(Kanto),
    HokurikuKoshinetsu(HokurikuKoshinetsu),
    Chubu(Chubu),
    Kinki(Kinki),
    ChugokuShikoku(ChugokuShikoku),
    KyusyuOkinawa(KyusyuOkinawa),
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
    Area::HokkaidoTohoku(HOKKAIDO_TOHOKU),
    Area::Kanto(KANTO),
    Area::HokurikuKoshinetsu(HOKURIKU_KOSHINETSU),
    Area::Chubu(CHUBU),
    Area::Kinki(KINKI),
    Area::ChugokuShikoku(CHUGOKU_SHIKOKU),
    Area::KyusyuOkinawa(KYUSYU_OKINAWA),
];

#[derive(Debug)]
pub struct HokkaidoTohoku {
    prefs: [Pref; 7],
}

const HOKKAIDO_TOHOKU: HokkaidoTohoku = HokkaidoTohoku {
    prefs: [
        Pref::Hokkaido,
        Pref::Aomori,
        Pref::Iwate,
        Pref::Miyagi,
        Pref::Akita,
        Pref::Yamagata,
        Pref::Fukushima,
    ],
};

#[derive(Debug)]
pub struct Kanto {
    prefs: [Pref; 7],
}

const KANTO: Kanto = Kanto {
    prefs: [
        Pref::Ibaraki,
        Pref::Tochigi,
        Pref::Gunma,
        Pref::Saitama,
        Pref::Chiba,
        Pref::Tokyo,
        Pref::Kanagawa,
    ],
};

pub struct HokurikuKoshinetsu {
    prefs: [Pref; 6],
}

const HOKURIKU_KOSHINETSU: HokurikuKoshinetsu = HokurikuKoshinetsu {
    prefs: [
        Pref::Niigata,
        Pref::Yamanashi,
        Pref::Nagano,
        Pref::Ishikawa,
        Pref::Toyama,
        Pref::Fukui,
    ],
};

#[derive(Debug)]
pub struct Chubu {
    prefs: [Pref; 4],
}

const CHUBU: Chubu = Chubu {
    prefs: [Pref::Aichi, Pref::Gifu, Pref::Shizuoka, Pref::Mie],
};

#[derive(Debug)]
pub struct Kinki {
    prefs: [Pref; 6],
}

const KINKI: Kinki = Kinki {
    prefs: [
        Pref::Osaka,
        Pref::Hyogo,
        Pref::Kyoto,
        Pref::Shiga,
        Pref::Nara,
        Pref::Wakayama,
    ],
};

#[derive(Debug)]
pub struct ChugokuShikoku {
    prefs: [Pref; 9],
}

const CHUGOKU_SHIKOKU: ChugokuShikoku = ChugokuShikoku {
    prefs: [
        Pref::Okayama,
        Pref::Hiroshima,
        Pref::Tottori,
        Pref::Shimane,
        Pref::Yamaguchi,
        Pref::Kagawa,
        Pref::Tokushima,
        Pref::Ehime,
        Pref::Kochi,
    ],
};

#[derive(Debug)]
pub struct KyusyuOkinawa {
    prefs: [Pref; 8],
}

const KYUSYU_OKINAWA: KyusyuOkinawa = KyusyuOkinawa {
    prefs: [
        Pref::Fukuoka,
        Pref::Saga,
        Pref::Nagasaki,
        Pref::Kumamoto,
        Pref::Oita,
        Pref::Miyazaki,
        Pref::Kagoshima,
        Pref::Okinawa,
    ],
};

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
