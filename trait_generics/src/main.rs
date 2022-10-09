mod example;

struct Canvas;
impl Canvas {
    fn write_at(&self, x: i32, y: i32, c: char) {}
}
// トレイトの定義
trait Visible {
    // メソッドのシグネチャのみ定義する
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;

    // トレイトにはデフォルト実装を持たせることもできる
    // デフォルト実装を持つメソッドはトレイト実装先で再実装する必要は無い
    fn default_impl_method() {
        println!("this is default implements");
    }
}

fn trait_declaration() {
    // 今回トレイトを実装するユーザ定義型
    struct Broom {
        x: i32,
        y: i32,
        height: i32,
    }
    // Broom型にVisibleトレイトを実装する
    impl Visible for Broom {
        // このimplブロックではVisibleトレイトの実装しか定義できない
        // 他にBroom型の型関連関数を定義したい場合はimpl Broom{}で記述する必要がある
        fn draw(&self, canvas: &mut Canvas) {
            for y in self.y - self.height - 1..self.y {
                canvas.write_at(self.x, y, '|');
            }
            canvas.write_at(self.x, self.y, 'M');
        }
        fn hit_test(&self, x: i32, y: i32) -> bool {
            self.x == x && self.y - self.height - 1 <= y && y <= self.y
        }
    }
}
// 拡張トレイト
fn extended_trait() {
    // ☆か否かを判定するメソッドを提供するトレイト
    trait IsStar {
        fn is_star(&self) -> bool;
    }
    // プリミティブ型であるcharにIsStarトレイトを実装する
    impl IsStar for char {
        fn is_star(&self) -> bool {
            self.eq(&'☆')
        }
    }
    // 既存の型にトレイトで拡張したメソッドを使うことができる
    assert_eq!('☆'.is_star(), true);

    // 以下のようにジェネリックimplブロックを書いて色々な型に拡張トレイトを追加できる
    // IsStarを実装する全ての型Tに対してIsStarトレイトの実装を行う、という意味になる
    // impl<T: IsStar> IsStar for T {
    //     fn is_star(&self) -> bool {
    //         false
    //     }
    // }
}

fn sub_trait() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    // 他のトレイトを別名で拡張することができる
    // CreatureトレイトはVisibleトレイトの拡張である
    // 全てのクリーチャーは可視であることを意味する
    trait Creature: Visible {
        fn position(&self) -> (i32, i32);
        fn facing(&self) -> Direction;
    }
    struct Broom;
    // Broom型にCreatureトレイトを実装する場合、CreatureのスーパートレイトであるVisibleトレイトの実装も必須
    // 各々のトレイトに対するimplブロックで実装する必要がある
    impl Visible for Broom {
        fn draw(&self, canvas: &mut Canvas) {}
        fn hit_test(&self, x: i32, y: i32) -> bool {
            true
        }
    }
    impl Creature for Broom {
        fn position(&self) -> (i32, i32) {
            (10, 20)
        }
        fn facing(&self) -> Direction {
            Direction::Down
        }
    }
}

fn main() {
    println!("Hello, world!");
    crate::example::trait_object_example();
}
