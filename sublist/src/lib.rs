#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
      return Comparison::Equal;
    }

    if _first_list.len() == 0 {
      return Comparison::Sublist;
    }

    if _second_list.len() == 0 {
      return Comparison::Superlist;
    }

    let first_is_longer = _first_list.len() > _second_list.len();
    let long_list;
    let short_list;

    if first_is_longer {
      long_list = _first_list;
      short_list = _second_list;
    } else {
      long_list = _second_list;
      short_list = _first_list;
    }

      if long_list.windows(short_list.len()).any(|window| window == short_list) {
        if first_is_longer {
          return Comparison::Superlist;
        } else {
          return Comparison::Sublist;
        }
      } else {
        return Comparison::Unequal;
      }
}
