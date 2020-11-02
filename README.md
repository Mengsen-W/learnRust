# recorder my learn rust language

## 1. 常见编程概念
  1. 变量默认不可变
  2. 函数参数必须指明类型，不关心函数定义在何处
  3. 语句是执行一些操作但不返回值的指令。表达式计算并产生一个值。函数调用是一个表达式。宏调用是一个表达式。我们用来创建新作用域的大括号（代码块）{}，也是一个表达式。

## 2. 所有权
  1. 对于确定长度的类型，可以直接在栈上创建对象，但是对于大小未知的类型，没办法直接在栈上创建对象，所以只能在堆上申请内存并且创建对象。
  2. 对于堆上部分的拷贝，rust既没有选择浅拷贝也就是复制指针，因为这样可能会导致未知的double free。也没有向C++一样采用深拷贝，即重新创建一个对象，因为这样可能导致性能上的损耗。rust的拷贝类似于c++指针移动，并销毁栈上的移动之前的指针
  3. rust同样也提供了深拷贝方法，clone()会直接复制堆上数据
  4. 对于函数传参和返回值，和拷贝的原理是一样的，总结起来就是：当持有堆中数据值的变量离开作用域时，其值将通过drop被清理掉，除非数据被移动为另一个变量所有
  5. 引用是一个不拥有所有权，但是存在使用权，用&表示引用，在rust中引用被称为借用，在借用中是不能修改变量的，即不允许修改引用的值
  6. 可变引用是rust中允许的一种操作，首先引用的对象必须是可变的变量，而且在特定作用域中的特定数据只能由一个可变引用
  7. 数据竞争可能由下面三个行为导致，全部满足的时候可能会出现数据竞争，产生未定义行为

     ​	两个或更多指针同时访问同一数据

     ​	至少有一个指针被用来写入数据

     ​	没有同步数据访问的机制

  8. rust为了避免数据竞争，做了如下几条规定

     可以允许拥有多个可变引用，只是不能同时拥有

     不能在拥有不可变引用的同时拥有可变引用

     一个引用的作用域从声明的地方开始一直持续到最后一次使用为止，作用域没有重叠

  9. rust用生命周期体系来避免悬垂引用
  10. 而是对部分的引用，字符串字面值就是slice，类型是&str
  11. 义一个获取字符串slice而不是String引用的函数使得我们的API更加通用并且不会丢失任何功能

## 3. 结构体

  1. 结构体中可以确实有堆上数据，或者只是存堆上引用
  2. 自动引用解引用，当使用结构体调用**方法**时，因为方法有一个明确的接受者，在给出接收者和方法名的前提下，Rust可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）

## 4. 错误处理

	1. 使用`panic!`宏来退出程序，`abort`或者栈展开
 	2. 用`match`和`Result`来处理一些可以恢复的错误
 	3. 使用`unwarp`生成自动错误信息或使用`expect`来生成自定义的错误信息
 	4. 可以将一些错误以`Result<T, S>`的形式返回给调用者
 	5. 用`?`运算符可以处理 `Result` 的值 ，与`match` 表达式有着完全相同的工作方式。如果 `Result` 的值是 `Ok`，这个表达式将会返回 `Ok` 中的值而程序将继续执行。如果值是 `Err`，`Err` 中的值将作为整个函数的返回值，就好像使用了 `return` 关键字一样，这样错误值就被传播给了调用者

## 5. 泛型

