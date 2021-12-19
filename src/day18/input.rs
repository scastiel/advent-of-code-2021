#[derive(Debug)]
pub enum Number {
    Reg(i64),
    Pair(Box<Number>, Box<Number>),
}

fn reg(number: i64) -> Number {
    Number::Reg(number)
}

fn pair(left: Number, right: Number) -> Number {
    Number::Pair(Box::new(left), Box::new(right))
}

#[allow(dead_code)]
pub fn example() -> Vec<Number> {
    vec![
        pair(
            pair(pair(reg(0), pair(reg(4), reg(5))), pair(reg(0), reg(0))),
            pair(
                pair(pair(reg(4), reg(5)), pair(reg(2), reg(6))),
                pair(reg(9), reg(5)),
            ),
        ),
        pair(
            reg(7),
            pair(
                pair(pair(reg(3), reg(7)), pair(reg(4), reg(3))),
                pair(pair(reg(6), reg(3)), pair(reg(8), reg(8))),
            ),
        ),
        pair(
            pair(reg(2), pair(pair(reg(0), reg(8)), pair(reg(3), reg(4)))),
            pair(
                pair(pair(reg(6), reg(7)), reg(1)),
                pair(reg(7), pair(reg(1), reg(6))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(2), reg(4)), reg(7)),
                pair(reg(6), pair(reg(0), reg(5))),
            ),
            pair(
                pair(pair(reg(6), reg(8)), pair(reg(2), reg(8))),
                pair(pair(reg(2), reg(1)), pair(reg(4), reg(5))),
            ),
        ),
        pair(
            reg(7),
            pair(reg(5), pair(pair(reg(3), reg(8)), pair(reg(1), reg(4)))),
        ),
        pair(
            pair(reg(2), pair(reg(2), reg(2))),
            pair(reg(8), pair(reg(8), reg(1))),
        ),
        pair(reg(2), reg(9)),
        pair(
            reg(1),
            pair(
                pair(pair(reg(9), reg(3)), reg(9)),
                pair(pair(reg(9), reg(0)), pair(reg(0), reg(7))),
            ),
        ),
        pair(pair(pair(reg(5), pair(reg(7), reg(4))), reg(7)), reg(1)),
        pair(
            pair(pair(pair(reg(4), reg(2)), reg(2)), reg(6)),
            pair(reg(8), reg(7)),
        ),
    ]
}

pub fn exercise() -> Vec<Number> {
    vec![
        pair(
            pair(
                pair(pair(reg(2), reg(5)), reg(4)),
                pair(pair(reg(1), reg(0)), pair(reg(8), reg(3))),
            ),
            pair(
                pair(reg(2), pair(reg(2), reg(4))),
                pair(reg(1), pair(reg(3), reg(3))),
            ),
        ),
        pair(
            pair(pair(reg(2), reg(2)), pair(pair(reg(4), reg(3)), reg(3))),
            pair(pair(pair(reg(8), reg(6)), reg(3)), pair(reg(3), reg(7))),
        ),
        pair(
            pair(pair(reg(9), pair(reg(4), reg(1))), pair(reg(9), reg(0))),
            pair(reg(6), pair(reg(6), reg(0))),
        ),
        pair(
            pair(
                pair(reg(3), reg(9)),
                pair(pair(reg(4), reg(4)), pair(reg(2), reg(5))),
            ),
            pair(pair(reg(9), pair(reg(8), reg(4))), reg(8)),
        ),
        pair(
            pair(
                pair(pair(reg(0), reg(0)), reg(9)),
                pair(pair(reg(9), reg(3)), pair(reg(8), reg(2))),
            ),
            pair(reg(2), pair(reg(1), reg(3))),
        ),
        pair(
            pair(pair(reg(8), reg(4)), reg(6)),
            pair(pair(reg(5), reg(1)), pair(reg(3), reg(6))),
        ),
        pair(
            pair(
                pair(reg(6), pair(reg(7), reg(6))),
                pair(pair(reg(2), reg(6)), reg(5)),
            ),
            pair(pair(reg(6), reg(4)), reg(2)),
        ),
        pair(
            pair(reg(1), pair(reg(9), reg(7))),
            pair(
                pair(pair(reg(5), reg(9)), pair(reg(9), reg(5))),
                pair(pair(reg(7), reg(0)), reg(1)),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(5), reg(8)), pair(reg(9), reg(4))),
                pair(pair(reg(9), reg(3)), pair(reg(7), reg(8))),
            ),
            reg(8),
        ),
        pair(
            pair(pair(reg(0), reg(9)), pair(pair(reg(6), reg(0)), reg(7))),
            pair(
                pair(pair(reg(7), reg(7)), reg(6)),
                pair(pair(reg(9), reg(7)), pair(reg(0), reg(4))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(4), reg(3)), pair(reg(9), reg(5))),
                pair(reg(7), pair(reg(7), reg(3))),
            ),
            pair(pair(pair(reg(2), reg(8)), reg(9)), reg(4)),
        ),
        pair(pair(reg(7), reg(5)), pair(reg(8), reg(1))),
        pair(
            pair(reg(4), reg(6)),
            pair(pair(pair(reg(0), reg(6)), reg(6)), pair(reg(7), reg(4))),
        ),
        pair(
            pair(
                pair(reg(1), reg(8)),
                pair(pair(reg(1), reg(4)), pair(reg(1), reg(6))),
            ),
            pair(reg(3), reg(4)),
        ),
        pair(
            pair(pair(reg(6), reg(5)), pair(reg(4), pair(reg(7), reg(3)))),
            pair(
                pair(pair(reg(0), reg(1)), pair(reg(8), reg(4))),
                pair(reg(4), reg(8)),
            ),
        ),
        pair(
            pair(reg(5), reg(1)),
            pair(reg(9), pair(reg(9), pair(reg(3), reg(3)))),
        ),
        pair(
            pair(pair(pair(reg(7), reg(0)), pair(reg(2), reg(5))), reg(1)),
            pair(reg(9), pair(pair(reg(2), reg(7)), pair(reg(4), reg(4)))),
        ),
        pair(
            pair(pair(pair(reg(5), reg(8)), reg(8)), reg(0)),
            pair(reg(8), pair(reg(1), pair(reg(2), reg(5)))),
        ),
        pair(reg(8), pair(pair(reg(5), reg(4)), reg(7))),
        pair(
            pair(pair(reg(9), reg(8)), pair(reg(6), reg(7))),
            pair(pair(reg(2), pair(reg(2), reg(6))), pair(reg(9), reg(6))),
        ),
        pair(
            pair(pair(pair(reg(2), reg(3)), reg(7)), reg(6)),
            pair(pair(reg(8), reg(6)), reg(3)),
        ),
        pair(
            pair(pair(reg(8), pair(reg(7), reg(2))), reg(3)),
            pair(pair(pair(reg(3), reg(9)), reg(4)), pair(reg(6), reg(8))),
        ),
        pair(
            reg(9),
            pair(
                pair(pair(reg(6), reg(7)), pair(reg(6), reg(0))),
                pair(pair(reg(3), reg(9)), reg(8)),
            ),
        ),
        pair(
            pair(pair(reg(7), reg(7)), pair(reg(4), reg(7))),
            pair(
                pair(pair(reg(9), reg(8)), reg(9)),
                pair(reg(9), pair(reg(2), reg(4))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(5), reg(0)), reg(1)),
                pair(reg(4), pair(reg(4), reg(8))),
            ),
            pair(reg(9), pair(reg(6), reg(7))),
        ),
        pair(
            pair(
                pair(pair(reg(9), reg(2)), reg(5)),
                pair(reg(1), pair(reg(5), reg(8))),
            ),
            pair(pair(reg(9), pair(reg(0), reg(1))), pair(reg(3), reg(8))),
        ),
        pair(
            pair(pair(reg(5), pair(reg(2), reg(5))), reg(8)),
            pair(reg(2), pair(reg(0), pair(reg(9), reg(3)))),
        ),
        pair(
            pair(reg(7), pair(pair(reg(8), reg(4)), pair(reg(8), reg(4)))),
            reg(4),
        ),
        pair(
            pair(
                pair(pair(reg(3), reg(3)), reg(4)),
                pair(pair(reg(0), reg(0)), pair(reg(5), reg(5))),
            ),
            pair(reg(4), reg(5)),
        ),
        pair(
            pair(pair(pair(reg(9), reg(3)), pair(reg(9), reg(3))), reg(2)),
            pair(reg(5), reg(3)),
        ),
        pair(
            pair(pair(reg(9), reg(5)), pair(reg(1), reg(4))),
            pair(pair(reg(7), reg(1)), pair(reg(3), pair(reg(6), reg(5)))),
        ),
        pair(
            reg(8),
            pair(
                pair(pair(reg(1), reg(1)), pair(reg(0), reg(1))),
                pair(reg(9), pair(reg(3), reg(6))),
            ),
        ),
        pair(
            pair(pair(pair(reg(4), reg(4)), reg(7)), pair(reg(0), reg(3))),
            pair(reg(1), reg(5)),
        ),
        pair(
            pair(pair(reg(3), pair(reg(0), reg(8))), reg(8)),
            pair(reg(5), pair(reg(7), reg(5))),
        ),
        pair(
            pair(pair(pair(reg(9), reg(6)), reg(2)), reg(7)),
            pair(pair(reg(5), pair(reg(3), reg(7))), reg(0)),
        ),
        pair(reg(4), reg(9)),
        pair(
            pair(
                pair(reg(5), pair(reg(1), reg(3))),
                pair(pair(reg(9), reg(5)), reg(6)),
            ),
            pair(pair(pair(reg(7), reg(9)), reg(5)), reg(3)),
        ),
        pair(
            pair(
                pair(pair(reg(3), reg(9)), pair(reg(7), reg(2))),
                pair(reg(5), pair(reg(8), reg(8))),
            ),
            pair(reg(1), reg(9)),
        ),
        pair(
            pair(
                pair(pair(reg(7), reg(8)), reg(8)),
                pair(pair(reg(9), reg(0)), pair(reg(5), reg(1))),
            ),
            pair(reg(6), pair(pair(reg(1), reg(0)), pair(reg(3), reg(3)))),
        ),
        pair(
            pair(
                pair(pair(reg(5), reg(8)), reg(1)),
                pair(pair(reg(8), reg(6)), pair(reg(2), reg(9))),
            ),
            pair(pair(reg(5), reg(1)), reg(6)),
        ),
        pair(
            pair(reg(1), reg(7)),
            pair(pair(reg(5), pair(reg(3), reg(2))), reg(4)),
        ),
        pair(
            pair(pair(pair(reg(3), reg(1)), reg(2)), pair(reg(0), reg(8))),
            pair(reg(3), pair(reg(4), reg(6))),
        ),
        pair(
            pair(reg(9), reg(6)),
            pair(reg(0), pair(pair(reg(5), reg(2)), pair(reg(1), reg(1)))),
        ),
        pair(
            pair(
                pair(pair(reg(1), reg(8)), reg(8)),
                pair(pair(reg(9), reg(0)), reg(3)),
            ),
            pair(
                pair(reg(6), pair(reg(2), reg(8))),
                pair(pair(reg(6), reg(4)), pair(reg(6), reg(0))),
            ),
        ),
        pair(
            pair(reg(7), pair(pair(reg(3), reg(2)), pair(reg(9), reg(0)))),
            pair(
                pair(pair(reg(3), reg(2)), pair(reg(2), reg(8))),
                pair(pair(reg(5), reg(5)), pair(reg(9), reg(2))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(2), reg(5)), pair(reg(3), reg(1))),
                pair(reg(7), pair(reg(9), reg(6))),
            ),
            pair(
                pair(pair(reg(7), reg(0)), reg(7)),
                pair(reg(2), pair(reg(9), reg(1))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(1), reg(6)), reg(9)),
                pair(reg(1), pair(reg(6), reg(5))),
            ),
            pair(pair(reg(8), pair(reg(4), reg(1))), reg(6)),
        ),
        pair(
            pair(
                pair(reg(7), pair(reg(4), reg(6))),
                pair(pair(reg(2), reg(7)), pair(reg(6), reg(6))),
            ),
            pair(reg(8), reg(0)),
        ),
        pair(
            pair(reg(9), reg(7)),
            pair(
                pair(pair(reg(0), reg(7)), reg(5)),
                pair(pair(reg(1), reg(4)), pair(reg(1), reg(3))),
            ),
        ),
        pair(
            pair(
                pair(reg(1), pair(reg(8), reg(2))),
                pair(pair(reg(0), reg(6)), pair(reg(9), reg(0))),
            ),
            reg(8),
        ),
        pair(
            pair(pair(reg(4), reg(0)), pair(reg(7), pair(reg(3), reg(3)))),
            pair(reg(9), reg(6)),
        ),
        pair(
            reg(0),
            pair(
                pair(pair(reg(6), reg(9)), reg(7)),
                pair(pair(reg(0), reg(6)), reg(1)),
            ),
        ),
        pair(
            reg(5),
            pair(
                pair(reg(4), reg(3)),
                pair(pair(reg(8), reg(3)), pair(reg(5), reg(7))),
            ),
        ),
        pair(
            pair(reg(9), reg(0)),
            pair(reg(0), pair(pair(reg(7), reg(8)), pair(reg(1), reg(8)))),
        ),
        pair(
            pair(pair(pair(reg(4), reg(3)), pair(reg(5), reg(6))), reg(2)),
            pair(pair(reg(2), reg(3)), reg(1)),
        ),
        pair(
            reg(4),
            pair(
                pair(reg(9), reg(9)),
                pair(pair(reg(1), reg(8)), pair(reg(9), reg(2))),
            ),
        ),
        pair(
            pair(pair(pair(reg(6), reg(9)), reg(5)), reg(1)),
            pair(pair(pair(reg(7), reg(4)), pair(reg(8), reg(1))), reg(3)),
        ),
        pair(
            pair(reg(8), pair(reg(5), pair(reg(2), reg(6)))),
            pair(pair(pair(reg(2), reg(7)), reg(6)), pair(reg(6), reg(0))),
        ),
        pair(
            pair(pair(pair(reg(6), reg(8)), reg(8)), reg(6)),
            pair(
                pair(pair(reg(5), reg(7)), reg(2)),
                pair(pair(reg(6), reg(5)), pair(reg(3), reg(0))),
            ),
        ),
        pair(
            pair(pair(reg(1), pair(reg(2), reg(5))), reg(3)),
            pair(reg(5), pair(reg(4), pair(reg(6), reg(6)))),
        ),
        pair(
            pair(pair(pair(reg(4), reg(9)), reg(8)), reg(1)),
            pair(reg(9), reg(0)),
        ),
        pair(
            pair(reg(1), pair(reg(0), pair(reg(5), reg(7)))),
            pair(
                pair(reg(1), pair(reg(5), reg(9))),
                pair(pair(reg(3), reg(2)), pair(reg(1), reg(7))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(2), reg(9)), pair(reg(2), reg(7))),
                pair(pair(reg(4), reg(2)), reg(5)),
            ),
            pair(
                pair(pair(reg(9), reg(1)), pair(reg(7), reg(2))),
                pair(reg(2), pair(reg(7), reg(5))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(5), reg(7)), pair(reg(8), reg(9))),
                pair(reg(5), pair(reg(7), reg(9))),
            ),
            pair(
                pair(reg(7), pair(reg(6), reg(6))),
                pair(reg(7), pair(reg(8), reg(0))),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(6), reg(6)), pair(reg(4), reg(6))),
                pair(reg(4), pair(reg(7), reg(8))),
            ),
            pair(reg(1), pair(pair(reg(5), reg(5)), pair(reg(1), reg(9)))),
        ),
        pair(
            pair(pair(pair(reg(4), reg(3)), reg(8)), reg(2)),
            pair(
                pair(reg(9), pair(reg(4), reg(0))),
                pair(reg(8), pair(reg(7), reg(0))),
            ),
        ),
        pair(
            pair(reg(2), pair(reg(7), reg(5))),
            pair(
                pair(pair(reg(0), reg(1)), reg(1)),
                pair(reg(8), pair(reg(3), reg(5))),
            ),
        ),
        pair(
            pair(
                pair(reg(4), pair(reg(4), reg(2))),
                pair(pair(reg(0), reg(4)), reg(9)),
            ),
            pair(reg(1), reg(4)),
        ),
        pair(
            pair(pair(reg(5), reg(5)), pair(reg(5), reg(6))),
            pair(
                pair(reg(0), pair(reg(4), reg(2))),
                pair(pair(reg(7), reg(8)), pair(reg(5), reg(6))),
            ),
        ),
        pair(
            reg(2),
            pair(
                pair(reg(0), pair(reg(9), reg(1))),
                pair(pair(reg(1), reg(7)), pair(reg(0), reg(0))),
            ),
        ),
        pair(pair(pair(reg(5), pair(reg(4), reg(8))), reg(1)), reg(9)),
        pair(reg(8), pair(pair(reg(2), reg(1)), pair(reg(3), reg(0)))),
        pair(
            pair(pair(pair(reg(6), reg(5)), pair(reg(1), reg(1))), reg(7)),
            pair(pair(pair(reg(7), reg(5)), reg(3)), pair(reg(0), reg(1))),
        ),
        pair(
            pair(pair(pair(reg(0), reg(3)), reg(7)), reg(7)),
            pair(
                pair(pair(reg(4), reg(8)), pair(reg(6), reg(1))),
                pair(pair(reg(6), reg(1)), reg(9)),
            ),
        ),
        pair(
            pair(pair(pair(reg(4), reg(8)), reg(9)), pair(reg(1), reg(0))),
            pair(reg(6), pair(reg(4), pair(reg(4), reg(8)))),
        ),
        pair(
            pair(pair(pair(reg(8), reg(0)), pair(reg(5), reg(1))), reg(6)),
            reg(1),
        ),
        pair(
            pair(
                pair(pair(reg(6), reg(6)), pair(reg(7), reg(7))),
                pair(pair(reg(4), reg(3)), pair(reg(2), reg(6))),
            ),
            pair(
                pair(reg(3), reg(5)),
                pair(pair(reg(7), reg(0)), pair(reg(7), reg(3))),
            ),
        ),
        pair(
            pair(reg(1), pair(reg(5), reg(8))),
            pair(
                pair(pair(reg(3), reg(7)), pair(reg(9), reg(6))),
                pair(pair(reg(4), reg(8)), pair(reg(3), reg(4))),
            ),
        ),
        pair(
            pair(pair(reg(1), reg(5)), pair(reg(8), reg(2))),
            pair(pair(pair(reg(3), reg(1)), reg(5)), pair(reg(4), reg(1))),
        ),
        pair(
            pair(pair(pair(reg(6), reg(3)), reg(5)), reg(8)),
            pair(
                pair(reg(9), pair(reg(3), reg(6))),
                pair(pair(reg(3), reg(5)), pair(reg(6), reg(9))),
            ),
        ),
        pair(
            pair(
                pair(reg(7), pair(reg(5), reg(4))),
                pair(reg(0), pair(reg(6), reg(0))),
            ),
            pair(
                pair(pair(reg(7), reg(7)), pair(reg(1), reg(1))),
                pair(pair(reg(5), reg(1)), reg(7)),
            ),
        ),
        pair(
            pair(pair(reg(1), reg(5)), pair(pair(reg(8), reg(6)), reg(0))),
            reg(5),
        ),
        pair(
            pair(
                pair(pair(reg(0), reg(8)), pair(reg(6), reg(0))),
                pair(pair(reg(3), reg(0)), reg(9)),
            ),
            pair(pair(pair(reg(7), reg(1)), reg(2)), pair(reg(4), reg(2))),
        ),
        pair(
            pair(
                pair(reg(6), pair(reg(8), reg(7))),
                pair(reg(2), pair(reg(2), reg(0))),
            ),
            pair(reg(9), pair(reg(7), pair(reg(6), reg(6)))),
        ),
        pair(
            reg(3),
            pair(
                pair(reg(7), pair(reg(4), reg(5))),
                pair(pair(reg(8), reg(5)), reg(4)),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(8), reg(0)), pair(reg(8), reg(3))),
                pair(pair(reg(5), reg(4)), pair(reg(1), reg(6))),
            ),
            pair(pair(reg(0), pair(reg(8), reg(5))), reg(3)),
        ),
        pair(
            pair(pair(reg(7), reg(2)), reg(1)),
            pair(reg(9), pair(pair(reg(3), reg(8)), reg(4))),
        ),
        pair(
            pair(reg(4), pair(reg(7), pair(reg(9), reg(9)))),
            pair(reg(3), reg(8)),
        ),
        pair(
            pair(
                pair(pair(reg(7), reg(1)), reg(9)),
                pair(pair(reg(6), reg(9)), pair(reg(9), reg(6))),
            ),
            pair(reg(2), reg(0)),
        ),
        pair(
            pair(
                pair(pair(reg(6), reg(2)), reg(9)),
                pair(reg(3), pair(reg(3), reg(9))),
            ),
            pair(pair(reg(8), pair(reg(3), reg(4))), pair(reg(3), reg(7))),
        ),
        pair(
            pair(reg(4), reg(9)),
            pair(reg(8), pair(reg(5), pair(reg(9), reg(8)))),
        ),
        pair(reg(3), pair(pair(reg(9), pair(reg(9), reg(7))), reg(4))),
        pair(
            pair(
                pair(pair(reg(5), reg(9)), reg(6)),
                pair(reg(1), pair(reg(3), reg(1))),
            ),
            pair(reg(4), pair(reg(1), pair(reg(3), reg(8)))),
        ),
        pair(
            pair(pair(pair(reg(7), reg(6)), reg(2)), reg(3)),
            pair(
                pair(reg(0), pair(reg(1), reg(8))),
                pair(pair(reg(4), reg(9)), pair(reg(4), reg(3))),
            ),
        ),
        pair(
            pair(reg(3), pair(pair(reg(8), reg(1)), pair(reg(3), reg(8)))),
            pair(
                pair(pair(reg(2), reg(0)), pair(reg(0), reg(8))),
                pair(pair(reg(7), reg(0)), reg(9)),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(9), reg(7)), pair(reg(9), reg(3))),
                pair(pair(reg(5), reg(8)), reg(6)),
            ),
            pair(pair(pair(reg(6), reg(2)), reg(0)), pair(reg(2), reg(4))),
        ),
        pair(
            pair(
                pair(reg(8), pair(reg(9), reg(7))),
                pair(pair(reg(5), reg(1)), pair(reg(1), reg(4))),
            ),
            reg(3),
        ),
        pair(
            pair(reg(7), pair(pair(reg(5), reg(6)), pair(reg(2), reg(7)))),
            pair(
                pair(pair(reg(7), reg(3)), reg(0)),
                pair(reg(1), pair(reg(0), reg(6))),
            ),
        ),
        pair(
            pair(reg(2), pair(pair(reg(5), reg(5)), reg(2))),
            pair(
                pair(reg(3), pair(reg(7), reg(2))),
                pair(pair(reg(7), reg(1)), reg(8)),
            ),
        ),
        pair(
            pair(
                pair(pair(reg(2), reg(4)), pair(reg(6), reg(8))),
                pair(reg(0), pair(reg(7), reg(5))),
            ),
            pair(pair(reg(3), pair(reg(2), reg(5))), pair(reg(7), reg(7))),
        ),
    ]
}
