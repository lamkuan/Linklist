use std::ptr;
use std::fmt;
use std::fmt::Formatter;

pub struct Node<T>
{
    pub value: T,
    pub next: *mut Node<T>,
    pub prev: *mut Node<T>,
}

impl<T> Node<T>
{
    pub fn new(value: T) -> Node<T>
    {
        Node {value, next: ptr::null_mut(), prev: ptr::null_mut()}
    }
}

pub struct LinkedList<T>
{
    head: *mut Node<T>,
}

impl<T> LinkedList<T>
{
    pub fn new(value: T) -> Self
    {
        unsafe {
            Self { head: Self::new_node(value) }
        }
    }

    /// 在鏈表頭部插入一個新節點
    pub fn push_front(&mut self, value: T)
    {
        unsafe {
            let new_node = Self::new_node(value);
            let next_node = (*self.head).next;
            (*self.head).next = new_node;
            (*new_node).prev = self.head;
            (*new_node).next = next_node;
            (*next_node).prev = new_node;
        }
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        unsafe {
            if self.is_null()
            {
                return None;
            }

            let pop_node = (*self.head).next;
            let next_node = (*pop_node).next;
            (*self.head).next = next_node;
            (*next_node).prev = self.head;

            Self::clear_node(pop_node);

            let boxed_node = Box::from_raw(pop_node);

            Some(boxed_node.value)
        }
    }

    /// 在鏈表尾部插入一個新節點
    pub fn push_back(&mut self, value: T)
    {
        unsafe {
            let new_node = Self::new_node(value);

            let prev_node = (*self.head).prev;

            (*new_node).next = self.head;
            (*new_node).prev = prev_node;
            (*prev_node).next = new_node;
            (*self.head).prev = new_node;
        }
    }

    pub fn pop_back(&mut self) -> Option<T>
    {
        unsafe {
            if self.is_null()
            {
                return None;
            }

            let pop_node = (*self.head).prev;
            let prev_node = (*pop_node).prev;

            (*prev_node).next = self.head;
            (*self.head).prev = prev_node;

            Self::clear_node(pop_node);

            let boxed_node = Box::from_raw(pop_node);

            Some(boxed_node.value)
        }
    }

    unsafe fn clear_node(node: *mut Node<T>)
    {
        (*node).next = ptr::null_mut();
        (*node).prev = ptr::null_mut();
    }

    unsafe fn new_node(value: T) -> *mut Node<T>
    {
        let new_node = Box::into_raw(Box::new(Node::new(value)));
        (*new_node).prev = new_node;
        (*new_node).next = new_node;

        new_node
    }

    unsafe fn is_null(&self) -> bool
    {
        (*self.head).next == self.head
    }
}

impl<T> fmt::Display for LinkedList<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut node = (*self.head).next;

            while node != self.head
            {
                write!(f, "{}", (*node).value)?;

                if (*node).next != self.head
                {
                    write!(f, "{}", "->")?;
                }

                node = (*node).next;
            }

            Ok(())
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}

        unsafe {
            let _ = Box::from_raw(self.head);
        }
    }
}
