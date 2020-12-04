/// chapter 8 collect
/// mengsen
/// 2020-11-30 10:39:41

fn main() {
  {
    learning_str();
  }
  {
    leaning_vec();
  }
}

fn learning_str() {
  {
    let tao = '道';
    let tao_u32 = tao as u32;
    assert_eq!(36947, tao_u32);
    assert_eq!(format!("U+{:x}", tao_u32), "U+9053");
    assert_eq!("\\u{9053}", tao.escape_unicode().to_string());
    assert_eq!(char::from(65), 'A');
    assert_eq!(std::char::from_u32(0x9053), Some('道'));
    assert_eq!(std::char::from_u32(36947), Some('道'));
    assert_eq!(std::char::from_u32(4294967295), None);

    let mut b = [0; 3];
    let tao = '道';
    let tao_str = tao.encode_utf8(&mut b);
    assert_eq!("道", tao_str);
    assert_eq!(3, tao.len_utf8());

    assert_eq!(true, 'f'.is_digit(16));
    assert_eq!(Some(15), 'f'.to_digit(16));
    assert!('a'.is_lowercase());
    assert!(!'道'.is_lowercase());
    assert!('A'.is_uppercase());
    assert!(!'a'.is_uppercase());
    assert!(!'中'.is_uppercase());
    assert_eq!("i", 'I'.to_lowercase().to_string());
    assert_eq!("B", 'b'.to_uppercase().to_string());
    assert!(' '.is_whitespace());
    assert!('\u{A0}'.is_whitespace());
    assert!(!'越'.is_whitespace());
    assert!('a'.is_alphabetic());
    assert!('京'.is_alphabetic());
    assert!(!'1'.is_alphabetic());
    assert!('7'.is_alphanumeric());
    assert!('K'.is_alphanumeric());
    assert!('藏'.is_alphanumeric());
    assert!(!'『'.is_alphanumeric());
    assert!(!' '.is_control());
    assert!(!'q'.is_control());
    assert!('7'.is_numeric());
    assert_eq!("\\r", '\r'.escape_default().to_string());
  }
  {
    let mut a = String::from("fooα");
    assert_ne!(format!("{:p}", a.as_ptr()), format!("{:p}", &a));
    assert_eq!(a.len(), 5);
    a.reserve(10);
    assert_eq!(a.capacity(), 15);
  }
  {
    let string: String = String::new();
    assert_eq!(string, "");
    let string: String = String::from("hello rust");
    assert_eq!("hello rust", string);
    let string: String = String::with_capacity(20);
    assert_eq!("", string);
    let _str: &'static str = "the tao of rust";
    let string: String = _str.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!("thetaoofrust", string);
    let string: String = _str.to_owned();
    assert_eq!("the tao of rust", string);
    let string: String = _str.to_string();
    let _str: &str = &string[11..15];
    assert_eq!("rust", _str);
  }
  {
    let word = "goodbye";
    let count = word.chars().count();
    assert_eq!(7, count);
    // return char iterator
    let mut chars = word.chars();
    assert_eq!(Some('g'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('d'), chars.next());
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('y'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(None, chars.next());

    // return bytes iterator
    let mut bytes = "bors".bytes();
    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());
    assert_eq!(None, bytes.next());
  }
  {
    let mut v = String::from("rust");
    assert_eq!(Some("ru"), v.get(0..2));
    assert_eq!(None, v.get(10..));
    assert_eq!(None, v.get_mut(10..));
    assert!(!v.is_char_boundary(10));

    let s = "Tao of Rust";
    let (first, last) = s.split_at(3);
    assert_eq!(first, "Tao");
    assert_eq!(last, " of Rust");
  }
  {
    let mut hello = String::from("Hello, ");
    hello.push('R');
    hello.push_str("ust");
    assert_eq!(hello, "Hello, Rust");

    let mut message = String::from("hello");
    message.extend([',', 'r', 'u'].iter());
    message.extend("st ".chars());
    message.extend("w o r l d".split_whitespace());
    assert_eq!("hello,rust world", message);

    let mut a = String::with_capacity(3);
    a.insert(0, 'f');
    a.insert(1, 'o');
    a.insert_str(0, "wms");
    assert_eq!("wmsfo", a);

    let left = "the tao".to_string();
    let mut right = "rust".to_string();
    assert_eq!(left + " of " + &right, "the tao of rust");
    right += "!";
    assert_eq!(right, "rust!");

    let s = String::from("fooαbar");
    let mut result = s.into_bytes();
    (0..result.len()).for_each(|i| {
      if i % 2 == 0 {
        result[i] = result[i].to_ascii_lowercase();
      } else {
        result[i] = result[i].to_ascii_uppercase();
      }
    });
    assert_eq!("fOoαBaR", String::from_utf8(result).unwrap());

    let s = String::from("fooαbar");
    let s: String = s
      .chars()
      .enumerate()
      .map(|(i, c)| {
        if i % 2 == 0 {
          c.to_lowercase().to_string()
        } else {
          c.to_uppercase().to_string()
        }
      })
      .collect();
    assert_eq!("fOoΑbAr", s);

    let mut s = String::from("hαllo");
    s.remove(3);
    assert_eq!("hαlo", s);
    assert_eq!(Some('o'), s.pop());
    assert_eq!(Some('l'), s.pop());
    assert_eq!(Some('α'), s.pop());
    assert_eq!("h", s);
    let mut s = String::from("hαllo");
    s.truncate(3);
    assert_eq!("hα", s);
    s.clear();
    assert_eq!(s, "");

    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    let t: String = s.drain(..beta_offset).collect();
    assert_eq!(t, "α is alpha, ");
    assert_eq!(s, "β is beta");
    s.drain(..);
    assert_eq!("", s);
  }
  {
    let ban = "bananas";
    assert!(ban.contains('a'));
    assert!(ban.contains("an"));
    assert!(ban.contains(char::is_lowercase));
    assert!(ban.starts_with('b'));
    assert!(!ban.ends_with("nana"));

    let s = "Hello, Rust: World!";
    assert_eq!(s.find('H'), Some(0));
    assert_eq!(s.find(','), Some(5));
    assert_eq!(s.rfind('!'), Some(18));
    assert_eq!(s.find("Rust"), Some(7));
    assert_eq!(s.find(char::is_whitespace), Some(6));
    assert_eq!(s.find(char::is_lowercase), Some(1));

    let s = "abc1defXghi";
    let v = s.split(|c| c == '1' || c == 'X').collect::<Vec<&str>>();
    assert_eq!(v, ["abc", "def", "ghi"]);

    let v = "Mary had a little lambda"
      .splitn(3, ' ')
      .collect::<Vec<&str>>();
    assert_eq!(v, ["Mary", "had", "a little lambda"]);
    let v = "A.B.".split(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "B", ""]);
    let v = "A.B.".split_terminator(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "B"]);
    let v = "A..B..".split(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "", "B", "", ""]);
    let v = "A..B..".split_terminator(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "", "B", ""]);

    let v = "abcXXXabcYYYabc".matches("abc").collect::<Vec<&str>>();
    assert_eq!(v, ["abc", "abc", "abc"]);
    let v = " 1abc2abc3 "
      .rmatches(char::is_numeric)
      .collect::<Vec<&str>>();
    assert_eq!(v, ["3", "2", "1"]);
    let v = "abcXXXabcYYYabc".match_indices("abc").collect::<Vec<_>>();
    assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
    let v = "abcXXXabcYYYabc".rmatch_indices("abc").collect::<Vec<_>>();
    assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

    let s = " Hello\tworld\t";
    assert_eq!("Hello\tworld", s.trim());
    assert_eq!("Hello\tworld\t", s.trim_start());
    assert_eq!(" Hello\tworld", s.trim_end());

    assert_eq!("Hello\tworld\t".trim_matches('\t'), "Hello\tworld");
    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
    assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");
    let x: &[char] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
    assert_eq!(
      "1foo1barXXX".trim_matches(|c| c == '1' || c == 'X'),
      "foo1bar"
    );
    assert_eq!(
      "123foo1bar123".trim_end_matches(char::is_numeric),
      "123foo1bar"
    );
    let x: &[char] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
    assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");

    let s = "Hello\tworld\t";
    assert_eq!("Hello world ", s.replace("\t", " "));
    assert_eq!("Hello world", s.replace("\t", " ").trim());
    let s = "this is old old 123";
    assert_eq!("this is new new 123", s.replace("old", "new"));
    assert_eq!("this is new old 123", s.replacen("old", "new", 1));
    assert_eq!("this is ald ald 123", s.replacen('o', "a", 3));
    assert_eq!(
      "this is old old new23",
      s.replacen(char::is_numeric, "new", 1)
    );
  }
  {
    use std::num::ParseIntError;
    use std::str::FromStr;
    #[derive(Debug, PartialEq)]
    struct Point {
      x: i32,
      y: i32,
    }
    impl FromStr for Point {
      type Err = ParseIntError;
      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
          .trim_matches(|p| p == '{' || p == '}')
          .split(',')
          .collect::<Vec<&str>>();
        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        Ok(Point {
          x: x_fromstr,
          y: y_fromstr,
        })
      }
    }

    {
      let p = Point::from_str("{1,2}");
      assert_eq!(p.unwrap(), Point { x: 1, y: 2 });
      let _err = Point::from_str("{3, u}");
      // Err (ParseintError { kind: InvalidD git })
    }
  }
  {
    {
      let s: String = format!("{}Rust", "Hello");
      assert_eq!(s, "HelloRust");
      assert_eq!(format!("{:5}", "HelloRust"), "HelloRust");
      assert_eq!(format!("{:5.3}", "HelloRust"), "Hel  ");
      assert_eq!(format!("{:10}", "HelloRust"), "HelloRust ");
      assert_eq!(format!("{:<12}", "HelloRust"), "HelloRust   ");
      assert_eq!(format!("{:>12}", "HelloRust"), "   HelloRust");
      assert_eq!(format!("{:^12}", "HelloRust"), " HelloRust  ");
      assert_eq!(format!("{:^12.5}", "HelloRust"), "   Hello    ");
      assert_eq!(format!("{:=^12.5}", "HelloRust"), "===Hello====");
      assert_eq!(format!("{:*^12.5}", "HelloRust"), "***Hello****");
    }
    {
      assert_eq!(format!("{:+}", 1234), "+1234");
      assert_eq!(format!("{:+}", -1234), "-1234");
      assert_eq!(format!("{:+#x}", 1234), "+0x4d2");
      assert_eq!(format!("{:b}", 1234), "10011010010");
      assert_eq!(format!("{:#b}", 1234), "0b10011010010");
      assert_eq!(format!("{:#20b}", 1234), "       0b10011010010");
      assert_eq!(format!("{:<#20b}", 1234), "0b10011010010       ");
      assert_eq!(format!("{:^#20b}", 1234), "   0b10011010010    ");
      assert_eq!(format!("{:>+#15x}", 1234), "         +0x4d2");
      assert_eq!(format!("{:>+#015x}", 1234), "+0x0000000004d2");
      assert_eq!(format!("{:<+#015x}", 1234), "+0x0000000004d2");
      assert_eq!(format!("{:0>+#15x}", 1234), "000000000+0x4d2");
    }
    {
      assert_eq!(format!("{:.4}", 1234.5678), "1234.5678");
      assert_eq!(format!("{:.5}", 1234.5678), "1234.56780");
      assert_eq!(format!("{:.2}", 1234.5618), "1234.56");
      assert_eq!(format!("{:.2}", 1234.5678), "1234.57");
      assert_eq!(format!("{:<10.4}", 1234.5678), "1234.5678 ");
      assert_eq!(format!("{:^12.2}", 1234.5678), "  1234.57   ");
      assert_eq!(format!("{:*^12.2}", 1234.5678), "**1234.57***");
      assert_eq!(format!("{:^012.2}", 1234.5678), "000001234.57");
      assert_eq!(format!("{:<012.2}", 1234.5678), "000001234.57");
      assert_eq!(format!("{:0^12.2}", 1234.5678), "001234.57000");
      assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");
    }
    use std::fmt::{self, Display, Formatter};
    struct City {
      name: &'static str,
      lat: f32,
      lon: f32,
    }
    impl Display for City {
      fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
          f,
          "{}: {:.3} {} {:3} {}",
          self.name,
          self.lat.abs(),
          lat_c,
          self.lon.abs(),
          lon_c
        )
      }
    }

    let city = City {
      name: "Beijing",
      lat: 39.90469,
      lon: -116.40717,
    };
    assert_eq!("Beijing: 39.905 N 116.40717 W", format!("{}", city));
  }
  {
    // diagonal sum
    let s = r"1234
            5678
            9876
            4321";
    let (mut x, mut y) = (0, 0);
    for (idx, val) in s.lines().enumerate() {
      // enumerate return idx and val tuple
      // trim black
      let val = val.trim();
      let left = val.get(idx..idx + 1).unwrap().parse::<u32>().unwrap();
      let right = val
        .get((3 - idx)..(3 - idx + 1))
        .unwrap()
        .parse::<u32>()
        .unwrap();

      x += left;
      y += right;
    }
    assert_eq!(38, x + y);
  }
}

fn leaning_vec() {
  let mut vec = Vec::new();
  vec.push(1);
  vec.push(2);
  assert_eq!(vec.len(), 2);
  assert_eq!(vec[0], 1);
  assert_eq!(vec.pop(), Some(2));
  assert_eq!(vec.len(), 1);
  vec[0] = 7;
  assert_eq!(vec[0], 7);
  assert_eq!(vec.get(0), Some(&7));
  assert_eq!(vec.get(10), None);
  // vec[lO] ;
  vec.extend([1, 2, 3].iter().cloned());
  assert_eq!(vec, [7, 1, 2, 3]);
  assert_eq!(vec.get(0..2), Some(&[7, 1][..]));
  let mut vec2 = vec![4, 5, 6];
  vec.append(&mut vec2);
  assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
  assert_eq!(vec2, []);
  vec.swap(1, 3);
  assert!(vec == [7, 3, 2, 1, 4, 5, 6]);
  let slice = [1, 2, 3, 4, 5, 6, 7];
  vec.copy_from_slice(&slice);
  assert_eq!(vec, slice);
  let slice = [4, 3, 2, 1];
  vec.resize(slice.len(), 0);
  vec.clone_from_slice(&slice);
  assert_eq!(vec, slice);

  let mut vec = Vec::with_capacity(10);
  for i in 0..10 {
    vec.push(i);
  }
  vec.truncate(0);
  assert_eq!(10, vec.capacity());
  for i in 0..10 {
    vec.push(i);
  }
  vec.clear();
  assert_eq!(10, vec.capacity());
  vec.shrink_to_fit();
  assert_eq!(0, vec.capacity());
  for i in 0..10 {
    vec.push(i);
    assert!(vec.capacity() > 0);
  }

  struct Foo;
  let mut vec = Vec::new();
  vec.push(Foo);
  // use max to mean 0
  assert_eq!(vec.capacity(), std::usize::MAX);

  let vec = [10, 40, 30];
  assert!(vec.contains(&30));
  assert!(!vec.contains(&50));
  assert!(vec.starts_with(&[10]));
  assert!(vec.starts_with(&[10, 40]));
  assert!(vec.ends_with(&[30]));
  assert!(vec.ends_with(&[40, 30]));
  assert!(vec.ends_with(&[]));
  let v: &[u8] = &[];
  assert!(v.starts_with(&[]));
  assert!(v.ends_with(&[]));

  let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
  assert_eq!(s.binary_search(&13), Ok(9));
  assert_eq!(s.binary_search(&4), Err(7));
  let r = s.binary_search(&1);
  assert!(match r {
    Ok(4) => true,
    _ => false,
  });
  let seek = 13;
  assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
  let s = [
    (0, 0),
    (2, 1),
    (4, 1),
    (5, 1),
    (3, 1),
    (1, 2),
    (2, 3),
    (4, 5),
    (5, 8),
    (3, 13),
    (1, 21),
    (2, 34),
    (4, 55),
  ];
  assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b), Ok(9));

  let mut v = [-5i32, 4, 1, -3, 2];
  v.sort();
  v.sort_unstable();
  assert!(v == [-5, -3, 1, 2, 4]);
  v.sort_by(|a, b| a.cmp(b));
  v.sort_unstable_by(|a, b| a.cmp(b));
  assert!(v == [-5, -3, 1, 2, 4]);
  v.sort_by(|a, b| b.cmp(a));
  assert!(v == [4, 2, 1, -3, -5]);
  v.sort_by_key(|k| k.abs());
  assert!(v == [1, 2, -3, 4, -5]);
}
