use std::fmt::Display;

pub enum Area {
    HokkaidoTohoku([Prefecture; 7]),
    Kanto([Prefecture; 7]),
    HokurikuKoshinetsu([Prefecture; 6]),
    Chubu([Prefecture; 4]),
    Kinki([Prefecture; 6]),
    ChugokuShikoku([Prefecture; 9]),
    KyusyuOkinawa([Prefecture; 8]),
}

impl Area {
    pub fn pref(&self) -> &[Prefecture] {
        use Area::*;
        match self {
            HokkaidoTohoku(v) => v,
            Kanto(v) => v,
            HokurikuKoshinetsu(v) => v,
            Chubu(v) => v,
            Kinki(v) => v,
            ChugokuShikoku(v) => v,
            KyusyuOkinawa(v) => v,
        }
    }
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

pub const AREA: [Area; 7] = [
    Area::HokkaidoTohoku([HOKKAIDO, AOMORI, IWATE, MIYAGI, AKITA, YAMAGATA, FUKUSHIMA]),
    Area::Kanto([IBARAKI, TOCHIGI, GUNMA, SAITAMA, CHIBA, TOKYO, KANAGAWA]),
    Area::HokurikuKoshinetsu([NIIGATA, YAMANASHI, NAGANO, ISHIKAWA, TOYAMA, FUKUI]),
    Area::Chubu([AICHI, GIFU, SHIZUOKA, MIE]),
    Area::Kinki([OSAKA, HYOGO, KYOTO, SHIGA, NARA, WAKAYAMA]),
    Area::ChugokuShikoku([
        OKAYAMA, HIROSHIMA, TOTTORI, SHIMANE, YAMAGUCHI, KAGAWA, TOKUSHIMA, EHIME, KOCHI,
    ]),
    Area::KyusyuOkinawa([
        FUKUOKA, SAGA, NAGASAKI, KUMAMOTO, OITA, MIYAZAKI, KAGOSHIMA, OKINAWA,
    ]),
];

#[derive(Debug, Clone, Copy)]
pub struct Prefecture {
    pub id: &'static str,
    pub name: &'static str,
    pub location: (f32, f32),
}

impl Prefecture {
    pub fn gen_gps(&self) -> String {
        let lat = self.location.0 + rand::random_range(-0.025..0.025);
        let long = self.location.1 + rand::random_range(-0.025..0.025);
        format!("{:.6},{:.6},pgs", lat, long)
    }
}

const HOKKAIDO: Prefecture = Prefecture {
    id: "JP1",
    name: "北海道",
    location: (43.064615, 141.346807),
};

const AOMORI: Prefecture = Prefecture {
    id: "JP2",
    name: "青森",
    location: (40.824308, 140.739998),
};

const IWATE: Prefecture = Prefecture {
    id: "JP3",
    name: "岩手",
    location: (39.703619, 141.152684),
};

const MIYAGI: Prefecture = Prefecture {
    id: "JP4",
    name: "宮城",
    location: (38.268837, 140.8721),
};

const AKITA: Prefecture = Prefecture {
    id: "JP5",
    name: "秋田",
    location: (39.718614, 140.102364),
};

const YAMAGATA: Prefecture = Prefecture {
    id: "JP6",
    name: "山形",
    location: (38.240436, 140.363633),
};

const FUKUSHIMA: Prefecture = Prefecture {
    id: "JP7",
    name: "福島",
    location: (37.750299, 140.467551),
};

const IBARAKI: Prefecture = Prefecture {
    id: "JP8",
    name: "茨城",
    location: (36.341811, 140.446793),
};

const TOCHIGI: Prefecture = Prefecture {
    id: "JP9",
    name: "栃木",
    location: (36.565725, 139.883565),
};

const GUNMA: Prefecture = Prefecture {
    id: "JP10",
    name: "群馬",
    location: (36.390668, 139.060406),
};

const SAITAMA: Prefecture = Prefecture {
    id: "JP11",
    name: "埼玉",
    location: (35.856999, 139.648849),
};

const CHIBA: Prefecture = Prefecture {
    id: "JP12",
    name: "千葉",
    location: (35.605057, 140.123306),
};

const TOKYO: Prefecture = Prefecture {
    id: "JP13",
    name: "東京",
    location: (35.689488, 139.691706),
};

const KANAGAWA: Prefecture = Prefecture {
    id: "JP14",
    name: "神奈川",
    location: (35.447507, 139.642345),
};

const NIIGATA: Prefecture = Prefecture {
    id: "JP15",
    name: "新潟",
    location: (37.902552, 139.023095),
};

const TOYAMA: Prefecture = Prefecture {
    id: "JP16",
    name: "富山",
    location: (36.695291, 137.211338),
};

const ISHIKAWA: Prefecture = Prefecture {
    id: "JP17",
    name: "石川",
    location: (36.594682, 136.625573),
};

const FUKUI: Prefecture = Prefecture {
    id: "JP18",
    name: "福井",
    location: (36.065178, 136.221527),
};

const YAMANASHI: Prefecture = Prefecture {
    id: "JP19",
    name: "山梨",
    location: (35.664158, 138.568449),
};

const NAGANO: Prefecture = Prefecture {
    id: "JP20",
    name: "長野",
    location: (36.651299, 138.180956),
};

const GIFU: Prefecture = Prefecture {
    id: "JP21",
    name: "岐阜",
    location: (35.391227, 136.722291),
};

const SHIZUOKA: Prefecture = Prefecture {
    id: "JP22",
    name: "静岡",
    location: (34.97712, 138.383084),
};

const AICHI: Prefecture = Prefecture {
    id: "JP23",
    name: "愛知",
    location: (35.180188, 136.906565),
};

const MIE: Prefecture = Prefecture {
    id: "JP24",
    name: "三重",
    location: (34.730283, 136.508588),
};

const SHIGA: Prefecture = Prefecture {
    id: "JP25",
    name: "滋賀",
    location: (35.004531, 135.86859),
};

const KYOTO: Prefecture = Prefecture {
    id: "JP26",
    name: "京都",
    location: (35.021247, 135.755597),
};

const OSAKA: Prefecture = Prefecture {
    id: "JP27",
    name: "大阪",
    location: (34.686297, 135.519661),
};

const HYOGO: Prefecture = Prefecture {
    id: "JP28",
    name: "兵庫",
    location: (34.691269, 135.183071),
};

const NARA: Prefecture = Prefecture {
    id: "JP29",
    name: "奈良",
    location: (34.685334, 135.832742),
};

const WAKAYAMA: Prefecture = Prefecture {
    id: "JP30",
    name: "和歌山",
    location: (34.225987, 135.167509),
};

const TOTTORI: Prefecture = Prefecture {
    id: "JP31",
    name: "鳥取",
    location: (35.503891, 134.237736),
};

const SHIMANE: Prefecture = Prefecture {
    id: "JP32",
    name: "島根",
    location: (35.472295, 133.0505),
};

const OKAYAMA: Prefecture = Prefecture {
    id: "JP33",
    name: "岡山",
    location: (34.661751, 133.934406),
};

const HIROSHIMA: Prefecture = Prefecture {
    id: "JP34",
    name: "広島",
    location: (34.39656, 132.459622),
};

const YAMAGUCHI: Prefecture = Prefecture {
    id: "JP35",
    name: "山口",
    location: (34.185956, 131.470649),
};

const TOKUSHIMA: Prefecture = Prefecture {
    id: "JP36",
    name: "徳島",
    location: (34.065718, 134.55936),
};

const KAGAWA: Prefecture = Prefecture {
    id: "JP37",
    name: "香川",
    location: (34.340149, 134.043444),
};

const EHIME: Prefecture = Prefecture {
    id: "JP38",
    name: "愛媛",
    location: (33.841624, 132.765681),
};

const KOCHI: Prefecture = Prefecture {
    id: "JP39",
    name: "高知",
    location: (33.559706, 133.531079),
};

const FUKUOKA: Prefecture = Prefecture {
    id: "JP40",
    name: "福岡",
    location: (33.606576, 130.418297),
};

const SAGA: Prefecture = Prefecture {
    id: "JP41",
    name: "佐賀",
    location: (33.249442, 130.299794),
};

const NAGASAKI: Prefecture = Prefecture {
    id: "JP42",
    name: "長崎",
    location: (32.744839, 129.873756),
};

const KUMAMOTO: Prefecture = Prefecture {
    id: "JP43",
    name: "熊本",
    location: (32.789827, 130.741667),
};

const OITA: Prefecture = Prefecture {
    id: "JP44",
    name: "大分",
    location: (33.238172, 131.612619),
};

const MIYAZAKI: Prefecture = Prefecture {
    id: "JP45",
    name: "宮崎",
    location: (31.911096, 131.423893),
};

const KAGOSHIMA: Prefecture = Prefecture {
    id: "JP46",
    name: "鹿児島",
    location: (31.560146, 130.557978),
};

const OKINAWA: Prefecture = Prefecture {
    id: "JP47",
    name: "沖縄",
    location: (26.2124, 127.680932),
};
