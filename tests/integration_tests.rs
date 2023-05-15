mod common;
use templater_rs::{builder::TemplaterBuilder, formatter::Formatter};

#[test]
fn test_readme_example() {
    struct Book {
        id: i64,
        title: String,
        author: String,
        contributors: Option<String>
    }

    let my_book = Book {
        id: 9784832275492,
        title: "Hidamari Sketch".to_owned(),
        author: "蒼樹うめ".to_owned(),
        contributors: None,
    };

    let format_string = "[%(id)] %(title) %(所有作者)";

    let templater = TemplaterBuilder::<Book>::new()
        .with_selector("id", |book| Some(book.id.to_string()))
        .with_selector("title", |book| Some(book.title.clone()))
        .with_selector("所有作者", |book| {
            Some(format!("(By: {}{})",
                &book.author,
                &book.contributors.clone().map(|x| format!(", {}", x)).or(Some("".to_owned())).unwrap())
            )
        })
        .build();

    {
        let result = templater.render(&my_book, format_string);
        assert!(result.is_ok());
        assert_eq!(
            result.ok().unwrap(),
            "[9784832275492] Hidamari Sketch (By: 蒼樹うめ)"
        );
    }
    {
        let formatter_result = Formatter::build(format_string);
        assert!(formatter_result.is_ok());
        let formatter = formatter_result.ok().unwrap();
        let result = templater.renderf(&my_book, &formatter);
        assert!(result.is_ok());
        assert_eq!(
            result.ok().unwrap(),
            "[9784832275492] Hidamari Sketch (By: 蒼樹うめ)"
        );
    }
}

#[test]
fn test_typical_workflow() {
    let templater = TemplaterBuilder::<common::ComplexObject>::new()
        .with_selector("a", |o| Some(o.a.to_string()))
        .with_selector("b", |o| Some(o.b.clone()))
        .with_selector("c", |o| o.c.clone())
        .with_selector("d", |o| Some(format!("{:?}", &o.d)))
        .with_selector("e", |o| Some(format!("{:?}", &o.e)))
        .with_selector("f", |o| Some(format!("({},{})", o.e.f.0, o.e.f.1)))
        .with_selector("g", |o| Some(format!("{:?}", &o.e.g)))
        .with_selector("h", |o| Some(format!("{:?}", &o.e.h)))
        .with_selector("i", |o| o.e.i.clone())
        .build();

    let obj = common::create_complex();
    let format_string = "|%(a)|%(b)|%(c)|%(d)|%(e)|%(f)|%(g)|%(h)|%(i)|";

    let result = templater.render(&obj, format_string);

    assert!(result.is_ok());
    assert_eq!(
        result.ok().unwrap(),
        "|64|a string|optional|[\"first\", \"second\"]|ComplexSubObject { f: (-2147, 2147), g: [], h: {\"hundred\": 100, \"ten thousand\": 10000, \"thousand\": 1000}, i: None }|(-2147,2147)|[]|{\"hundred\": 100, \"ten thousand\": 10000, \"thousand\": 1000}|NA|"
    );
}
