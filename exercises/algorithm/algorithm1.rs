/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 修复 merge 函数，移除冗余的泛型参数 T
    pub fn merge(
        list_a: &LinkedList<T>,
        list_b: &LinkedList<T>,
    ) -> LinkedList<T> where T: Ord + Display + Clone {
        // 创建一个新的空链表用于存储合并结果
        let mut merged = LinkedList::new();
        
        // 获取两个链表的头部节点
        let mut a_curr = list_a.start;
        let mut b_curr = list_b.start;
        
        // 当两个链表都还有节点时，比较它们的值并将较小的添加到合并列表
        while let (Some(a), Some(b)) = (a_curr, b_curr) {
            unsafe {
                let a_val = &(*a.as_ptr()).val;
                let b_val = &(*b.as_ptr()).val;
                
                // 比较值并将较小的添加到合并列表
                if a_val <= b_val {
                    merged.add(a_val.clone());
                    // 移动a_curr到下一个节点
                    a_curr = (*a.as_ptr()).next;
                } else {
                    merged.add(b_val.clone());
                    // 移动b_curr到下一个节点
                    b_curr = (*b.as_ptr()).next;
                }
            }
        }
        
        // 处理链表a中剩余的节点
        while let Some(a) = a_curr {
            unsafe {
                merged.add((*a.as_ptr()).val.clone());
                a_curr = (*a.as_ptr()).next;
            }
        }
        
        // 处理链表b中剩余的节点
        while let Some(b) = b_curr {
            unsafe {
                merged.add((*b.as_ptr()).val.clone());
                b_curr = (*b.as_ptr()).next;
            }
        }
        
        merged
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

/*
	binary search 
	This problem requires you to implement a binary search algorithm
*/

pub fn binary_search<T: Ord>(array: &[T], target: &T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        
        match target.cmp(&array[mid]) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => {
                if mid == 0 {
                    return None;
                }
                high = mid - 1;
            },
            std::cmp::Ordering::Greater => {
                low = mid + 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(&list_a, &list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(&list_a, &list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}