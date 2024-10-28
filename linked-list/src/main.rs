#![allow(dead_code, unused)]

#[derive(Clone, Debug)]
struct LinkedList {
    value: Option<i32>,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            value: None,
            next: None,
        }
    }

    #[allow(warnings)]
    fn push(&mut self, value: i32) {
        match &mut self.next {
            None => {
                self.next = Some(Box::new(LinkedList {
                    value: Some(value),
                    next: None,
                }));
            }
            Some(ref mut next_node) => {
                next_node.push(value);
            }
        }
    }

    #[allow(warnings)]
    fn pop(&mut self) -> Option<i32> {
        match &mut self.next {
            None => {
                self.value.take()
            }
            Some(ref mut next_node) => {
                if next_node.next.is_none() {
                    let last_value = next_node.value.take();

                    self.next = None;
                    last_value
                }else {
                    next_node.pop()
                }
            }
        }
    }
}

fn find_same_values_ll(list1: &mut LinkedList, list2: &mut LinkedList) -> i32 {
    let mut i: i32 = 0;
    while let (Some(v1), Some(v2)) = (list1.pop(), list2.pop()) {
        println!("{} {}", v1 , v2);
        if v1 == v2 {
            return i;
        }
        i+=1;
    }

    -1
} 

fn main() {
    let mut list1 = LinkedList::new();
    let mut list2 = LinkedList::new();

    list1.push(10);
    list1.push(10);
    list1.push(30);
    list1.push(30);

    list2.push(20);
    list2.push(15);
    list2.push(10);
    list2.push(20);

    let result = find_same_values_ll(&mut list1, &mut list2);

    println!("the first combined element is index {:?}", result);
}

