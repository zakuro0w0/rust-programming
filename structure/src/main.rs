fn named_field_structure() {
    // 名前付きフィールド型構造体
    // 構造体の名前はアッパーキャメルケースまたはパスカルケース
    struct GrayScaleMap {
        // フィールドやメソッドの名前は小文字のスネークケース
        pixels: Vec<u8>,
        size: (usize, usize),
    }

    let width = 1024;
    let height = 576;
    // 構造体の初期化方法1
    // C++と異なりフィールド名を明示する必要がある
    let image = GrayScaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    // 構造体の初期化方法2
    // フィールド名と同じ名前の変数や引数を並べる方法もある
    fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayScaleMap {
        match true {
            // 変数名とフィールド名が一致している場合、構造体初期化に与える順序は関係無い
            // 同名のフィールドの初期値として使われる
            true => GrayScaleMap { pixels, size },
            false => GrayScaleMap { size, pixels },
        }
    }
    // 構造体インスタンスのフィールドへのアクセスは.で行う
    println!("{}", image.pixels.len());
    println!("{}", image.size.0);

    // 構造体はデフォルトでprivateなので宣言されたモジュールとそのサブモジュールからしか参照できない
    // モジュール外部からも構造体にアクセスしたい場合はpubキーワードを付ける必要がある
    pub struct GrayScaleMap2 {
        // 同様にフィールドもデフォルトでprivateなので、モジュール外部に公開したい場合はpubを付ける
        pub pixels: Vec<u8>,
        pub size: (usize, usize),
    }

    // 構造体のみ公開し、フィールドはモジュール外部に公開しない方法もある
    // 但し、構造体式で初期化するには構造体の全てのフィールドが公開されている必要があるため、
    // GrayScaleMap3をモジュール外部から構造体式で初期化することはできない
    // Vec::new()のようにpublicなインスタンス構築用のメソッドを提供する必要がある
    pub struct GrayScaleMap3 {
        pixels: Vec<u8>,
        size: (usize, usize),
    }
}

// 同じ型の他の変数をベースにして構造体を初期化する方法
fn initialize_structure_omit_expression() {
    // ほうき型モンスターができる活動
    #[derive(Copy, Clone)]
    enum BroomIntent {
        FetchWater,
        DumpWater,
    }
    // ほうき型のモンスターを表現する構造体
    struct Broom {
        name: String,
        height: u32,
        health: u32,
        position: (f32, f32, f32),
        intent: BroomIntent,
    }

    // 1本のほうきを半分に折って2本のほうきを返す関数
    fn chop(b: Broom) -> (Broom, Broom) {
        // 引数のほうきをベースに、長さだけ半分にする
        let mut broom1 = Broom {
            height: b.height / 2,
            // height以外は ..式に与えた変数と同じ値を持たせる
            ..b
        };
        let mut broom2 = Broom {
            // StringはCopyではないので明示的にcloneする必要がある
            name: broom1.name.clone(),
            // name以外は最初に半分に折ったほうきと同じ値で初期化する
            ..broom1
        };
        // 1本目のほうきの名前に I を付ける
        broom1.name.push_str(" I");
        // 2本目のほうきの名前に II を付ける
        broom2.name.push_str(" II");
        // 2本のほうきを返す
        (broom1, broom2)
    }

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::DumpWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey2.height, 30);
}

fn main() {
    println!("Hello, world!");
    initialize_structure_omit_expression();
}
