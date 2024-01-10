use std::{rc::Rc, cell::RefCell, collections::VecDeque};

#[derive(Debug, Clone, Copy)]
pub enum Errors {

    // TOML file IO
    FSReadError,
    NoTOMLFilesFoundError,
    ParseTOMLFilError,
    TOMLFileIsEmpty,

    // URL
    RequestGetError,
    URLParseError,
    
    ParseTextError,

    PairAlreadyExistsError,

    LookupDeletionFailedError,
    NothingFoundError,

    SelectorGenerateError,
    ParseTitleError,

    LookupFailedError,


    WriteToStdoutError,

    CursorPosOverflowError,
}



#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // both empty
    if (root1.is_none() & root2.is_none()) == true { return true; }
    // one is empty, the other is not
    if (root1.is_none() ^ root2.is_none()) == true { return false; }

    let mut q1 = VecDeque::with_capacity(200);
    let mut q2 = q1.clone();

    

    q1.push_back(root1.unwrap());
    q2.push_back(root2.unwrap());

    loop {
        // both empty
        if q1.is_empty() && q2.is_empty() {
            println!("true because q1 q2 both empty");
            return true;
        }
        // one is empty but other is not
        if (!q1.is_empty() & !q2.is_empty()) == false {
            println!("false because q1 q2 one is empty, q1 e? {}, q2 e? {}", q1.is_empty(), q2.is_empty());
            return false;
        }


        let mut q1_n = None;
        while let Some(n) = q1.pop_front() {
            let mut n = n.borrow_mut();

            // this is an END node
            if n.left.is_none() && n.right.is_none() {
                q1_n = Some(n.val);
                break;
            }

            // otherwise: must deal with right side (if present) first to ensure order
            if n.right.is_some() {
                q1.push_front(n.right.take().unwrap());
            }

            if n.left.is_some() {
                q1.push_front(n.left.take().unwrap());
            }
        }

        let mut q2_n = None;
        while let Some(n) = q2.pop_front() {
            let mut n = n.borrow_mut();

            // this is an END node
            if n.left.is_none() && n.right.is_none() {
                q2_n = Some(n.val);
                break;
            }

            // otherwise: must deal with right side (if present) first to ensure order
            if n.right.is_some() {
                q2.push_front(n.right.take().unwrap());
            }

            if n.left.is_some() {
                q2.push_front(n.left.take().unwrap());
            }
        }
        println!("curr q1 {:?}, q2 {:?}", q1_n, q2_n);
        if q1_n != q2_n {
            return false;
        }
    }
}


use std::collections::{HashMap, HashSet};


pub fn jan9_m2385(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {

    const MAX_LEN: usize = 10usize.pow(5);

    if root.is_none() { return 0; }

    // translate the BT into HM first

    let mut connex = HashMap::with_capacity(MAX_LEN);
    let mut queue = VecDeque::with_capacity(MAX_LEN);

    queue.push_back(root.unwrap());

    while let Some(node) = queue.pop_front() {
        let mut node = node.borrow_mut();

        let (left, right) = (node.left.take(), node.right.take());

        [left, right]
        .into_iter()
        .for_each(|n| {
            if let Some(n) = n {

                connex
                .entry(node.val)
                .and_modify(|s: &mut HashSet<i32>| { s.insert(n.borrow().val); })
                .or_insert(HashSet::from([n.borrow().val]));


                connex
                .entry(n.borrow().val)
                .and_modify(|s: &mut HashSet<i32>| { s.insert(node.val); })
                .or_insert(HashSet::from([node.val]));

                queue.push_front(n);
            }
        });
    }

    let mut ans = 0;
    // translation complete, now start the zombie apocalypse
    let mut curr_infectors = HashSet::with_capacity(connex.len());
    let mut infected = HashSet::with_capacity(connex.len());

    curr_infectors.insert(start);
    infected.insert(start);

    while !connex.is_empty() {
        curr_infectors
        .drain()
        .for_each(|inf| {
            let infectenda = connex.remove(&inf);
            if let Some(infectenda) = infectenda {
                infectenda
                .into_iter()
                .for_each(|new| {
                    if !infected.contains(&new) {
                        curr_infectors.insert(new);
                    }

                    infected.insert(new);
                });
            }
        });

        ans += 1;
    }
    
    ans
}




// pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
//     if root.is_none() {
//         return 0;
//     }

//     let range = low..=high;

//     let mut queue = VecDeque::with_capacity(10usize.pow(5));
//     queue.push_back(root.unwrap().borrow_mut());
//     let mut count = 0;

//     while let Some(mut node) = queue.pop_front() {
//         if range.contains(&node.val) {
//             count += 1;
//         }

//         [node.left.take(), node.right.take()]
//             .into_iter()
//             .for_each(|mut n|  {
//                 if n.as_ref().is_some_and(|n| range.contains(&n.borrow().val)) {
//                     queue.push_back(n.unwrap().borrow_mut());
//                 }
//             });
//     }

//     count
// }