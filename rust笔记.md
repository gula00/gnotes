18错误处理

捕获，传播（延迟？），返回消息

haskell 语言是干什么的，有什么应用领域，语言有什么特点，给一些使用例子

> People do numerical work in Haskell, just not as commonly as in Python, Julia, MATLAB, R, or Fortran.

go 携带err对象

保证异常安全：避免抛出异常

​	mutex没来得及释放，就catch error了

返回err需要立即处理，但是我们可以用函数式编程简化

rs学习haskell

​	maybe - option

​	either - result

?可以只传播错误而不立即处理（最后map_error），但是不能不同类型错误，需要from trait

panic!是严重的错误，需要立即暴露；unwarp()和expect()无法转换的时候也会出现

catch_unwind()捕获并被转换panic为一个 Result

thiserror/anyhow库简化error建模

#must_use



19闭包

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```

闭包本质上是一个特殊的struct

存在栈上，其他语言会产生堆内存分配性能低

​	rust从根本上使用所有权和借用，没有move时符合引用借用规则，有move的时候生命周期和闭包一致

&string的标准结构，ptr|cap|len

```rust
// After this, name1 and table belong to c4
let c4 = move || println!("hello: {}, {:?}", name1, table);

c4();   // prints using the moved values
// name1;  // error: value moved into the closure
```

FnOnce

​	call_once使用self，转移

隐式：

```rust
let c = move |greeting: String| (greeting, name);
```

FnMut

​	&mut self可以call多次

Fn

​	&self

闭包练习题

```rust
pub trait Executor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str>;
}

struct BashExecutor {
    env: String,
}

impl Executor for BashExecutor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!(
            "fake bash execute: env: {}, cmd: {}",
            self.env, cmd
        ))
    }
}

// 看看我给的 tonic 的例子，想想怎么实现让 27 行可以正常执行

fn main() {
    let env = "PATH=/usr/bin".to_string();

    let cmd = "cat /etc/passwd";
    let r1 = execute(cmd, BashExecutor { env: env.clone() });
    println!("{:?}", r1);

    let r2 = execute(cmd, |cmd: &str| {
        Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
    });
    println!("{:?}", r2);
}

fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
    exec.execute(cmd)
}
```

答案

```rust
impl<F> Executor for F
where
    F: Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
{
    fn execute(&self, cmd: &str) -> Result<...> {
        self(cmd)
    }
}
```

这里面execute命名重复，一个是普通函数一个是trait方法



20如何阅读源码

trait → struct → 函数 / 方法

- 先从需求的流程中敲定系统的行为，需要定义什么接口 trait

- 再考虑系统有什么状态，定义了哪些数据结构 struct

- 最后到实现细节，包括如何为数据结构实现 trait、数据结构自身有什么算法、如何把整个流程串起来等等



15智能指针

和胖指针&str的区别：String对堆上的值有所有权，可以Deref/DerefMut/Drop

Box::new()是一个函数，会先分配到stack上然后传到heap上，需要inline优化

Cow clone-on-write 包裹一个只读借用，但如果调用者需要所有权或者需要修改内容，那么它会 clone 借用的数据

```rust
use std::borrow::Cow;

use url::Url;
fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    // 因为 k, v 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 Borrowed
    println!("key: {}, v: {}", k, v);
    // 当修改发生时，k 变成 Owned
    k.to_mut().push_str("_lala");

    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());
    // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
    // 所以这里 value 是 Owned
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v),
    }
}
```

MutexGuard

有点难，遇到了再看



cons list has infinite size

直接存list不知道大小，用box可以因为只是存pointer，而且存在heap上更容易扩展

```rust
enum List {
    // Cons(i32, List),
  	Cons(i32, Box<>List),
    Nil,
}
```

rust compiler会自动做padding

也可以用Rc，这样可以反复使用比如Rc::clone(&a)



String::from的意义：把&str转化成String，&str不拥有，这样可以在heap上分配内存

&String -> &str 强制类型转换，因为有deref返回值实现

从具体转为抽象，从mut转为immutable

&str是16字节胖指针 |ptr|len|，len已知所以可以放在stack上，没有裸str

String 24字节struct |ptr|len|cap| 指向heap

&String是指向String的8字节普通指针

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
  	hello(&m);
}
```



impl drop 不能显示调用



Rc::strong_count

还有weak_count避免**循环**

> The parent keeps the child alive. The child can look at the parent, but does not keep it alive.

RefCell内部可变性，不安全

Rc和RefCell都只适用于单线程

 

i32 as usize 必须要转化，好麻烦
