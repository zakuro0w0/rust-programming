trait SomeTrait {
    fn name(&self) -> String;
}

struct GenericsList<T: SomeTrait> {
    elements: Vec<T>,
}

impl<T: SomeTrait> GenericsList<T> {
    fn print(&self) {
        for t in &self.elements {
            println!("{}", t.name());
        }
    }
    fn add(&mut self, t: T) {
        self.elements.push(t);
    }
}

struct TraitObjectList {
    elements: Vec<Box<dyn SomeTrait>>,
}

impl TraitObjectList {
    fn print(&self) {
        for some_trait in &self.elements {
            println!("{}", some_trait.name());
        }
    }
    fn add(&mut self, element: Box<dyn SomeTrait>) {
        self.elements.push(element);
    }
}

struct MyStructure1;
struct MyStructure2;

impl SomeTrait for MyStructure1 {
    fn name(&self) -> String {
        "MyStructure1".to_string()
    }
}

impl SomeTrait for MyStructure2 {
    fn name(&self) -> String {
        "MyStructure2".to_string()
    }
}

pub fn generics_example() {
    let x = MyStructure1;
    let y = MyStructure2;
    // 次のコードはコンパイルエラーを起こす
    // mismatched types
    // expected struct `MyStructure1`, found struct `MyStructure2`rustcE0308
    //
    // これはMyList::elementsに格納する型Tを変数xのMyStructure1で確定した後に
    // 変数yのMyStructure2型を並べようとしたため。
    // genericsで書いたコードはコンパイル時に型が一意に決まる(=単相化される)ため、
    // 型Tを「SomeTraitを実装した任意の型」と表現していても実際には
    // 具体的な1つの型に特定しなければコンパイルできないことに注意する必要がある。
    // 単相化されても問題無く動くようなパターンであればgenericsで実現できるが、
    // そうでないパターンの場合はgenericsではなくtrait objectを使う必要がある
    // let z = MyList { elements: vec![x, y] };
}

pub fn trait_object_example() {
    // trait objectはBox::newで実体型を包んだ後、
    // dynキーワードを付けたTraitのBoxで受け取る必要がある(少し手間がかかる)
    let x: Box<dyn SomeTrait> = Box::new(MyStructure1);
    let y: Box<dyn SomeTrait> = Box::new(MyStructure2);
    // genericsパターンとは違い、実体型の異なる変数x, yを1つのVecに並べることができる。
    // これはtrait objectがコンパイル時点で単相化されるgenericsとは異なり、
    // 実行時に動的にキャストされるコストを受け入れることで得られるメリットの1つである。
    // trait objectパターンでないと実現できないデザインパターンもある。
    // StateパターンやCommandパターンは同じTraitを実装した複数の実体型を
    // 1つの変数に上書きしたり、同じ配列に並べたりするためである。
    let z = TraitObjectList {
        elements: vec![x, y],
    };
    z.print();
}
