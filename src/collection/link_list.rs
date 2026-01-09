//!
//! Persistent Linked List for clonables.
//! NOTE: this is pretty inefficient because it does
//!       lots of cloning().  If you have a large
//!       struct as T then you will want to wrap it
//!       in an RC() to avoid a lot copying.
//!
use std::{borrow::Borrow, fmt::Debug, rc::Rc};


// A link in a linked list.
// If the wrapped Option is a Some then
// then the link leads to the next node in the list.
// Otherwise, when the wrapped option is a None,
// the the link is a terminal link.
type Link<T> = Rc<Option<LinkNode<T>>>;

// a node in a linked list
#[derive(Debug, Clone, PartialEq)]
struct LinkNode<T> {
    elem: T,
    tail: Link<T>,
}

impl <T> LinkNode<T> {
    // construct a terminal node
    fn null() -> Rc<Option<T>> {
        Rc::new(None)
    }
}

// A linked list.
// This structure wraps the head node
// and the length of the list.
// This allows us to return the
// length of the list in constant time.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkList<T> {
    size: usize,
    list: Link<T>,
}

impl <T> LinkList<T> where T: Clone + Debug + PartialEq {
    ///
    /// Create a new empty list
    ///
    pub fn new() -> LinkList<T> {
        LinkList::<T>{size: 0, list: LinkNode::null()}
    }

    pub fn of_one(elem: T) -> LinkList<T> {
        LinkList::new().insert(elem)
    }
    pub fn of_two(elem: T, elem2: T) -> LinkList<T> {
        LinkList::new().insert(elem2).insert(elem)
    }
    pub fn of_three(elem: T, elem2: T, elem3: T) -> LinkList<T> {
        LinkList::new().insert(elem3).insert(elem2).insert(elem)
    }
    pub fn of_four(elem: T, elem2: T, elem3: T, elem4: T) -> LinkList<T> {
        LinkList::new().insert(elem4).insert(elem3).insert(elem2).insert(elem)
    }
    pub fn of_five(elem: T, elem2: T, elem3: T, elem4: T, elem5: T) -> LinkList<T> {
        LinkList::new().insert(elem5).insert(elem4).insert(elem3).insert(elem2).insert(elem)
    }
    pub fn of_six(elem: T, elem2: T, elem3: T, elem4: T, elem5: T, elem6: T) -> LinkList<T> {
        LinkList::new().insert(elem6).insert(elem5).insert(elem4).insert(elem3).insert(elem2).insert(elem)
    }
    pub fn of_seven(elem: T, elem2: T, elem3: T, elem4: T, elem5: T, elem6: T, elem7: T) -> LinkList<T> {
        LinkList::new().insert(elem7).insert(elem6).insert(elem5).insert(elem4).insert(elem3).insert(elem2).insert(elem)
    }
    pub fn of_eight(elem: T, elem2: T, elem3: T, elem4: T, elem5: T, elem6: T, elem7: T, elem8: T) -> LinkList<T> {
        LinkList::new().insert(elem8).insert(elem7).insert(elem6).insert(elem5).insert(elem4).insert(elem3).insert(elem2).insert(elem)
    }

    ///
    /// Determine if the list is empty
    ///
    pub fn is_empty(&self) -> bool {
        self.list.is_none()
    }

    ///
    /// Determine if the list is not empty
    ///
    pub fn is_not_empty(&self) -> bool {
        self.list.is_some()
    }

    ///
    /// Number of nodes in the list
    ///
    pub fn len(&self) -> usize {
        self.size
    }

    ///
    /// Get the element at the head of the list
    ///
    pub fn head(&self) -> Option<T> {
        match self.list.as_ref() {
            Some(node) => {
                Some(node.elem.clone())
            },
            None => None,
        }
    }

    ///
    /// Get the list's tail (the list after the head element)
    /// - The empty list has no tail, so this returns an option
    ///
    pub fn tail(&self) -> Option<LinkList<T>> {
        match self.list.as_ref() {
            Some(node) => {
                match &node.tail.borrow() {
                    Some(_) => {
                        Some(LinkList{size: self.size - 1, list: node.tail.clone()})
                    },
                    None => Some(LinkList::new()),  // empty list
                }
            },
            None => None,
        }
    }

    ///
    /// Insert an element at the head of the list
    ///
    pub fn insert(&self, elem: T) -> LinkList<T> {
        match self.list.as_ref() {
            Some(_) => {
                LinkList{size: self.size + 1, list: Rc::new(Some(LinkNode{elem: elem, tail: self.list.clone()}))}
            },
            None => {
                LinkList{size: 1, list: Rc::new(Some(LinkNode{elem: elem, tail: LinkNode::null()}))}
            },
        }
    }

    /**
     * Insert an element at the given index in the list.
     * If the index >= the size of the list, then
     * the element is appended.
     *
     * @param element the element to insert
     * @param index the index to insert at
     * @return new list with the element inserted if index < size
     *         else with element appended if index >= size
     */
    pub fn insert_at(&self, index: usize, elem: T) -> LinkList<T>
    {
        if index >= self.len() {
            self.append(elem)
        }
        else if index == 0 {
            self.insert(elem)
        } else {
            // find ith element
            let mut list = self.clone();
            let mut left = LinkList::new();
            let mut i = 0;
            while i < index {
                left = left.insert(list.head().unwrap());
                list = list.tail().unwrap();
                i += 1;
            }

            // insert element and then re-insert left
            list = list.insert(elem);
            while !left.is_empty() {
                list = list.insert(left.head().unwrap());
                left = left.tail().unwrap();
            }
            list
        }
    }
    ///
    /// append an element to the list
    ///
    pub fn append(&self, elem: T) -> LinkList<T> {
        self.reverse().insert(elem).reverse()
    }

    ///
    /// Reverse the list
    ///
    pub fn reverse(&self) -> LinkList<T> {
        match self.list.as_ref() {
            Some(head) => {
                let mut reversed = Rc::new(Some(LinkNode{elem: head.elem.clone(), tail: LinkNode::null()}));
                let mut list = head;
                loop {
                    match list.tail.as_ref() {
                        None => break,
                        Some(tail) => {
                            reversed = Rc::new(Some(LinkNode{elem: tail.elem.clone(), tail: reversed}));
                            list = tail;
                        },
                    }
                }
                LinkList{size: self.size, list: reversed}
            },
            None => LinkList::new(),
        }
    }

    ///
    /// concatenate two lists
    ///
    pub fn concat(&self, other: &LinkList<T>) -> LinkList<T> {
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            // neither list is empty
            let mut list = self.reverse();
            let mut other_list = other.clone();
            while !other_list.is_empty() {
                list = list.insert(other_list.head().unwrap());
                other_list = other_list.tail().unwrap();
            }
            list.reverse()
        }
    }

    /**
     * Remove the element at the given index.
     * If the index is past the end of the list,
     * then the list is returned.
     *
     * @param index
     * @return list without element at index
     */
    pub fn remove_at(&self, index: usize) -> LinkList<T>
    {
        if self.is_empty() || index >= self.len() {
            return self.clone()
        }

        //
        // iterate to avoid recursive calls
        // loop will use insert to build intermediate list to avoid many calls to append.
        // the result is then reversed.
        //
        let mut left = LinkList::new();
        let mut list = self.clone();
        let mut i = 0;
        while i < index {
            left = left.insert(list.head().unwrap());
            list = list.tail().unwrap();
            i += 1;
        }
        // pop ith element
        list = list.tail().unwrap();

        // re-insert left
        while !left.is_empty() {
            list = list.insert(left.head().unwrap());
            left = left.tail().unwrap();
        }
        list
    }

    /**
     * Get the sublist starting at the nth element.
     *
     * @param n
     * @return list starting at nth element or empty list if n >= length
     */
    pub fn nth(&self, n: usize) -> LinkList<T>{
        if n >= self.len() {
            return LinkList::new();
        }

        let mut list = self.clone();
        let mut i: usize = 0;
        while (i < n) && !list.is_empty() {
            list = list.tail().unwrap();
            i += 1;
        }
        return list;
    }

    /**
     * Given a list, create a new list with two elements swapped.
     *
     * @param list original list
     * @param i ith element is moved to jth position
     * @param j jth element is move to ith position
     * @param <T> type of elements
     * @return new list with ith and jth elements swapped
     *         if jth is out of range, then this acts
     *         like remove_at(i)
     */
    pub fn swap(&self, i: usize, j: usize) -> LinkList<T> {
        if self.is_empty() { return LinkList::new() }
        if i == j { return self.clone() } // nothing to swap
        if i > j { return self.swap(j, i) }  // want i < j
        if i >= self.len() { return self.clone() }
        if j >= self.len() { return self.remove_at(i) }

        //
        // 1. get list up to ith element
        // 2. get ith element
        // 3. get list up to jth element
        // 4. get jth element
        // 5. get list to tail
        // 6. piece it back together, swapping ith and jth element
        //
        let mut list = self.clone();
        let mut left = LinkList::new();

        // 1. get list up to ith element
        let mut k: usize = 0;
        while (k < i) && !list.is_empty() {
            left = left.insert(list.head().unwrap());
            list = list.tail().unwrap();
            k += 1;
        }

        // 2. get ith element
        let ith = list.head().unwrap();
        list = list.tail().unwrap();
        k += 1;

        // 3. get list up to jth element
        let mut middle = LinkList::new();
        while (k < j) && !list.is_empty() {
            middle = middle.insert(list.head().unwrap());
            list = list.tail().unwrap();
            k += 1;
        }

        // 4. get jth element
        let jth = list.head().unwrap();

        // 5. get list to tail
        list = list.tail().unwrap();

        //
        // 6. piece it back together, swapping ith and jth element
        //
        list = list.insert(ith);    // swap ith and jth
        while !middle.is_empty() {
            list = list.insert(middle.head().unwrap());
            middle = middle.tail().unwrap();
        }

        list = list.insert(jth);    // swap ith and jth
        while !left.is_empty() {
            list = list.insert(left.head().unwrap());
            left = left.tail().unwrap();
        }
        list
    }

    /**
     * Generate factorial(n) permutations of n length list.
     *
     * NOTE: this only maintains unique permutations, so
     *       the output my be less than factorial(n) in size.
     *
     * for instance, given [a b c d] it produces;
     *     a b c d
     *     a b d c
     *     a c b d
     *     a c d b
     *     a d b c
     *     a d c b
     *     b a c d
     *     b a d c
     *     b c a d
     *     b c d a
     *     c a b d
     *     c a d b
     *     c b a d
     *     c b d a
     *     c d a b
     *     c d b a
     *     d a b c
     *     d a c b
     *     d b a c
     *     d b c a
     *     d c a b
     *     d c b a
     *
     * @param list the list to permute
     * @param <T> the type of elements in the list
     * @return a set of lists which represent permutations of the input list
     */
    pub fn permute(&self) -> LinkList<LinkList<T>>
    {
        let mut results = LinkList::<LinkList<T>>::new();

        if self.is_empty() {
            // empty list has zero permutations
        }
        else if self.len() == 1 {
            // single element list has one permutation
            results = results.insert(self.clone());
        }
        else
        {
            //
            // 1. i = i;
            // 2. left = ith, right = removeAt(i);
            // 2. get all permutations of the right
            // 3. combine left + right permutations
            // 4. combine right permutations + left.
            // 6. i += 1
            //
            let mut i = 0;
            let mut left = self.clone();
            while !left.is_empty() {
                let mut right = self.remove_at(i).permute();

                //
                // add all head + tails
                //
                while !right.is_empty() {
                    results = results.insert(right.head().unwrap().insert(left.head().unwrap()));
                    right = right.tail().unwrap();
                }

                i = i + 1;
                left = left.tail().unwrap();
            }
        }
        return results;
    }

    /**
     * Find the given element is the list
     *
     * @param element the element to find
     * @return the subslist that starts with the element
     *         or empty list if the element is not found.
     */
    pub fn find(&self, element: &T) -> LinkList<T> {
        let mut list = self.clone();
        while !list.is_empty() {
            if list.head().unwrap() == *element {
                return list;
            }
            list = list.tail().unwrap();
        }
        return list;
    }

    /**
     * Map the values in the list using the mapper function
     * and return a new list.
     *
     * @param mapper function that maps a T to an R
     * @param <R> the result type
     * @return list of elements mapped from T to R
     */
    pub fn map<R>(&self, mapper: fn(&T) -> R) -> LinkList<R>
        where R: Clone + Debug + PartialEq
    {
        if self.is_empty() {
            return LinkList::<R>::new();
        }

        //
        // iterate to avoid recursion.
        // we build a reversed list of mapped elements
        // using insertion to avoid the extra list scans
        // that append would incur.
        // then we just un-reverse the mapped list at the end.
        //
        let mut mapped_list = LinkList::<R>::new();
        let mut list = self.clone();
        while !list.is_empty() {
            // inserts mapped values in reverse order
            mapped_list = mapped_list.insert(mapper(&list.head().unwrap()));
            list = list.tail().unwrap();
        }
        mapped_list.reverse() // un-reverse it.
    }

    /**
     * Filter a list given a predicate.
     *
     * @param predicate
     * @return a new list with those elements where predicate.test() returns true.
     */
    pub fn filter(&self, predicate: fn(&T) -> bool) -> LinkList<T>
    {
        if self.is_empty() {
            return self.clone();
        }

        //
        // iterate to avoid recursive calls
        // loop will use insert to build intermediate list to avoid many calls to append.
        // the result is then reversed.
        //
        let mut return_list = LinkList::new();
        let mut list = self.clone();
        while !list.is_empty() {
            let head = list.head().unwrap();
            if predicate(&head) {
                return_list = return_list.insert(head);
            }
            list = list.tail().unwrap();
        }
        return_list.reverse()
    }

    //
    // convert LinkList to LinkList of LinkList.
    // (this is the inverse of flatten)
    //
    pub fn fatten(&self) -> LinkList<LinkList<T>> {
        let mut return_list = LinkList::<LinkList<T>>::new();
        let mut list = self.clone();
        while !list.is_empty() {
            let head = list.head().unwrap();
                return_list = return_list.insert(LinkList::new().insert(head));
            list = list.tail().unwrap();
        }
        return_list.reverse()
    }
}

impl <T> LinkList<LinkList<T>> where T: Clone + Debug + PartialEq {
    //
    // convert a LinkList of LinkList into a LinkList
    //
    pub fn flatten(&self) -> LinkList<T> {
        if self.is_empty() {
            return LinkList::new()
        }
        let mut return_list = LinkList::<T>::new();
        let mut list = self.clone();
        while !list.is_empty() {
            return_list = return_list.concat(&list.head().unwrap());
            list = list.tail().unwrap();
        }
        return_list
    }

    /**
     * Map the values in the list using the mapper function
     * and flatten the resulting list of lists.
     *
     * @param mapper maps a T to a list of R
     * @param <R> the result type of the list
     * @return flattened list of R
     */
    pub fn flatmap<R>(&self, mapper: fn(&T) -> R) -> LinkList<R>
        where R: Clone + Debug + PartialEq
    {
        if self.is_empty() {
            return LinkList::<R>::new()
        }
        let mut return_list = LinkList::<R>::new();
        let mut list = self.clone();
        while !list.is_empty() {
            return_list = return_list.concat(&list.head().unwrap().map(mapper));
            list = list.tail().unwrap();
        }
        return_list
    }

    /**
     * Create all combinations of the elements of a fat list and
     * return the set of combinations
     *
     * so combine([a, b], [c, d], [e, f]) produces 2^3 = 8 combinations
     * [[a, c, e], [a, c, f], [a, d, e], [a, d, f], [b, c, e], [b, c, f], [b, d, e] [b, d, f]]
     *
     * @param <T>
     * @return
     */
    pub fn combinations(&self) -> LinkList<LinkList<T>>
    {
        self.accumulate_combinations(LinkList::new())
    }

    //
    // private recursive helper that takes an accumulator
    //
    fn accumulate_combinations(&self, accumulator:LinkList<LinkList<T>>) ->  LinkList<LinkList<T>>
    {
        if self.is_empty() {
            return accumulator.clone();
        }

        // we have at least two lists to combine
        self.tail().unwrap().accumulate_combinations(accumulator.combine(self.head().unwrap()))
    }

	/**
     * Create all combinations of appending the list elements to
     * the lists in the fat list self.
     *
     * So combineAppend([['a', 'b', 'c'], ['d', 'e', 'f']], ['1', '2', '3'])
     * produces [['a', 'b', 'c', '1'], ['a', 'b', 'c', '2'], ['a', 'b', 'c', '3'],
     *           ['d', 'e', 'f', '1'], ['d', 'e', 'f', '2'], ['d', 'e', 'f', '3']]
     *
     * @param accumulator
     * @param list
     * @param <T>
     * @return
     */
    pub fn combine(&self, flatlist: LinkList<T>) -> LinkList<LinkList<T>>
    {
        if flatlist.is_empty() {
            return self.clone();
        }

        if self.is_empty() {
            return flatlist.fatten();
        }

        //
        // generate all the combinations of appending elements of list
        // to the lists in accumulator.
        //
        let mut combinations = LinkList::<LinkList<T>>::new();
        let mut fatlist = self.clone();
        while !fatlist.is_empty()
        {
            let fathead = fatlist.head().unwrap();
            let mut list = flatlist.clone();
            while !list.is_empty() {
                combinations = combinations.insert(fathead.append(list.head().unwrap()));
                list = list.tail().unwrap();
            }

            fatlist = fatlist.tail().unwrap();
        }

        combinations
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = LinkList::<i32>::new();
        println!("{:#?}", list);

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
    }

    #[test]
    fn test_list_of_one() {
        let one = "one".to_string();
        let list = LinkList::<String>::new();
        let list = list.insert(one.clone());
        println!("{:#?}", list);

        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list.head().unwrap(), one);
        assert_eq!(list.tail().unwrap(), LinkList::new()); // empty list
    }

    #[test]
    fn test_list_of_two() {
        let one = "one".to_string();
        let two = "two".to_string();
        let list = LinkList::<String>::new();
        let list = list.insert(two.clone()).insert(one.clone());
        println!("{:#?}", list);

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list.head().unwrap(), one);
        assert_eq!(list.tail().unwrap().head().unwrap(), two);
    }

    #[test]
    fn test_insert_at() {
        let list = LinkList::<i32>::new().append(1).append(2);
        assert_eq!(list.insert_at(0, 42), LinkList::<i32>::new().append(42).append(1).append(2));
        assert_eq!(list.insert_at(1, 42), LinkList::<i32>::new().append(1).append(42).append(2));
        assert_eq!(list.insert_at(2, 42), LinkList::<i32>::new().append(1).append(2).append(42));

        assert_eq!(LinkList::<i32>::new().insert_at(0, 42), LinkList::<i32>::new().append(42));
    }

    #[test]
    fn test_equality() {
        //
        // empty lists are equal
        //
        assert_eq!(LinkList::<i32>::new(), LinkList::<i32>::new());

        //
        // identical lists are equal
        //
        let list1 = LinkList::<i32>::new().insert(2).insert(1);  // 1,2
        let list2 = LinkList::<i32>::new().insert(2).insert(1);  // 1,2
        assert_eq!(list1, list2);

        //
        // non-empty list is NOT equal to the empty list
        //
        assert_ne!(list1, LinkList::<i32>::new());

        //
        // non-identical lists are NOT equal
        //
        let list1 = LinkList::<i32>::new().insert(2).insert(1);  // 1,2
        let list2 = LinkList::<i32>::new().insert(3).insert(4);  // 3,4
        assert_ne!(list1, list2);
    }

    #[test]
    fn test_size() {
        let list = LinkList::<i32>::new();
        assert_eq!(list.len(), 0);

        let list = list.insert(1);
        assert_eq!(list.len(), 1);
        let list = list.insert(2);
        assert_eq!(list.len(), 2);
        let list = list.insert(3);
        assert_eq!(list.len(), 3);

        let list = list.tail().unwrap();
        assert_eq!(list.len(), 2);
        let list = list.tail().unwrap();
        assert_eq!(list.len(), 1);
        let list = list.tail().unwrap();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_reverse() {
        let list = LinkList::<i32>::new();
        let list = list.insert(2).insert(1);
        let list = list.reverse();

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list, LinkList::<i32>::new().insert(1).insert(2));
        assert_eq!(list.head().unwrap(), 2);
        assert_eq!(list.tail().unwrap().head().unwrap(), 1);
    }

    #[test]
    fn test_reverse_rc() {
        //
        // We can wrap a non-clonable in Rc.
        // We can wrap a clonable in Rc so we don't make copies.
        //
        #[derive(Debug, PartialEq)] // not clonable
        struct Data {
            value: String
        }
        let one = Data{value: "one".to_string()};
        let two = Data{value: "two".to_string()};
        // this won't compile: let list = LinkList::<Data>::new();
        let list = LinkList::<Rc<Data>>::new();
        let list = list.insert(Rc::new(two)).insert(Rc::new(one));
        let list = list.reverse();

        let one = Data{value: "one".to_string()};
        let two = Data{value: "two".to_string()};
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list.head().unwrap().as_ref(), &two);
        assert_eq!(list.tail().unwrap().head().unwrap().as_ref(), &one);
    }

    #[test]
    fn test_append() {
        let one = "one".to_string();
        let two = "two".to_string();
        let three = "three".to_string();
        let list = LinkList::<String>::new();

        //
        // appending to empty list
        //
        let list = list.append(one.clone());
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list.head().unwrap(), one);

        //
        // appending to list of one
        //
        let list = list.append(two.clone());
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list.head().unwrap(), one);
        assert_eq!(list.tail().unwrap().head().unwrap(), two);

        //
        // appending to list of two
        //
        let list = list.append(three.clone());
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert!(list.list.is_some());
        assert_eq!(list.head().unwrap(), one);
        assert_eq!(list.tail().unwrap().head().unwrap(), two);
        assert_eq!(list.tail().unwrap().tail().unwrap().head().unwrap(), three);
    }

    #[test]
    fn test_concatenate() {
        let list1 = LinkList::<i32>::new().insert(2).insert(1);  // 1,2
        let list2 = LinkList::<i32>::new().insert(4).insert(3);  // 3,4

        let list = list1.concat(&list2); // 1,2,3,4
        assert_eq!(list.len(), 4);
        assert!(!list.is_empty());
        assert_eq!(list.head(), Some(1));
        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(2));
        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(3));
        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(4));
        let list = list.tail().unwrap();
        assert!(list.is_empty());

        //
        // concatenating empty lists is empty
        //
        assert!(LinkList::<i32>::new().concat(&LinkList::<i32>::new()).is_empty());
        assert_eq!(LinkList::<i32>::new().concat(&LinkList::<i32>::new()), LinkList::<i32>::new());

        //
        // concatenating to an empty list is the other list
        //
        let list = LinkList::<i32>::new().concat(&list1);
        assert_eq!(list.len(), list1.len());
        assert!(!list.is_empty());
        assert_eq!(list, list1);
        assert_eq!(list.head(), Some(1));
        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(2));
        let list = list.tail().unwrap();
        assert!(list.is_empty());

        //
        // concatenating an empty list is the original list
        //
        let list = list1.concat(&LinkList::<i32>::new());
        assert_eq!(list.len(), list1.len());
        assert!(!list.is_empty());
        assert_eq!(list, list1);
        assert_eq!(list.head(), Some(1));
        let list = list.tail().unwrap();
        assert_eq!(list.head(), Some(2));
        let list = list.tail().unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_at() {
        let list = LinkList::<i32>::new().append(1).append(2).append(3);

        assert_eq!(list.remove_at(0), LinkList::<i32>::new().append(2).append(3));
        assert_eq!(list.remove_at(1), LinkList::<i32>::new().append(1).append(3));
        assert_eq!(list.remove_at(2), LinkList::<i32>::new().append(1).append(2));
        assert_eq!(list.remove_at(3), list);
    }

    #[test]
    fn test_nth() {
        let list = LinkList::<i32>::new().append(1).append(2).append(3);

        assert_eq!(list.nth(0), list);
        assert_eq!(list.nth(1), LinkList::<i32>::new().append(2).append(3));
        assert_eq!(list.nth(2), LinkList::<i32>::new().append(3));
        assert!(list.nth(3).is_empty());

    }

    #[test]
    fn test_swap() {
        let list = LinkList::<i32>::new().append(1).append(2).append(3).append(4);

        let swapped = list.swap(1, 2);
        assert_eq!(swapped,
            LinkList::<i32>::new().append(1).append(3).append(2).append(4)
        );
        let swapped = list.swap(0, 3);
        assert_eq!(swapped,
            LinkList::<i32>::new().append(4).append(2).append(3).append(1)
        );

        // order of indices does not matter
        let swapped = list.swap(2, 1);
        assert_eq!(swapped,
            LinkList::<i32>::new().append(1).append(3).append(2).append(4)
        );

        // one index out of range is same as remove_at(i)
        let swapped = list.swap(1, 4);
        assert_eq!(swapped,
            LinkList::<i32>::new().append(1).append(3).append(4)
        );

        // both indices out of range
        let swapped = list.swap(4, 5);
        assert_eq!(swapped, list);

        // empty list results in empty list
        assert_eq!(LinkList::<i32>::new().swap(1, 2), LinkList::<i32>::new());
    }

    #[test]
    fn test_permute() {
        let list = LinkList::<String>::new().append("A".to_string()).append("B".to_string()).append("C".to_string()).append("D".to_string());
        let permutations = list.permute();

        let mut permutation = permutations.clone();
        while !permutation.is_empty() {
            println!("{:?}", permutation.head().unwrap());
            permutation = permutation.tail().unwrap();
        }

        assert_eq!(24, permutations.len());

        let values = LinkList::<LinkList::<String>>::new()
        .insert(LinkList::<String>::new().append("A".to_string()).append("B".to_string()).append("C".to_string()).append("D".to_string()))
        .insert(LinkList::<String>::new().append("A".to_string()).append("B".to_string()).append("D".to_string()).append("C".to_string()))
        .insert(LinkList::<String>::new().append("A".to_string()).append("C".to_string()).append("B".to_string()).append("D".to_string()))
        .insert(LinkList::<String>::new().append("A".to_string()).append("C".to_string()).append("D".to_string()).append("B".to_string()))
        .insert(LinkList::<String>::new().append("A".to_string()).append("D".to_string()).append("B".to_string()).append("C".to_string()))
        .insert(LinkList::<String>::new().append("A".to_string()).append("D".to_string()).append("C".to_string()).append("B".to_string()))

        .insert(LinkList::<String>::new().append("B".to_string()).append("A".to_string()).append("C".to_string()).append("D".to_string()))
        .insert(LinkList::<String>::new().append("B".to_string()).append("A".to_string()).append("D".to_string()).append("C".to_string()))
        .insert(LinkList::<String>::new().append("B".to_string()).append("C".to_string()).append("A".to_string()).append("D".to_string()))
        .insert(LinkList::<String>::new().append("B".to_string()).append("C".to_string()).append("D".to_string()).append("A".to_string()))
        .insert(LinkList::<String>::new().append("B".to_string()).append("D".to_string()).append("A".to_string()).append("C".to_string()))
        .insert(LinkList::<String>::new().append("B".to_string()).append("D".to_string()).append("C".to_string()).append("A".to_string()))

        .insert(LinkList::<String>::new().append("C".to_string()).append("A".to_string()).append("B".to_string()).append("D".to_string()))
        .insert(LinkList::<String>::new().append("C".to_string()).append("A".to_string()).append("D".to_string()).append("B".to_string()))
        .insert(LinkList::<String>::new().append("C".to_string()).append("B".to_string()).append("A".to_string()).append("D".to_string()))
        .insert(LinkList::<String>::new().append("C".to_string()).append("B".to_string()).append("D".to_string()).append("A".to_string()))
        .insert(LinkList::<String>::new().append("C".to_string()).append("D".to_string()).append("A".to_string()).append("B".to_string()))
        .insert(LinkList::<String>::new().append("C".to_string()).append("D".to_string()).append("B".to_string()).append("A".to_string()))

        .insert(LinkList::<String>::new().append("D".to_string()).append("A".to_string()).append("B".to_string()).append("C".to_string()))
        .insert(LinkList::<String>::new().append("D".to_string()).append("A".to_string()).append("C".to_string()).append("B".to_string()))
        .insert(LinkList::<String>::new().append("D".to_string()).append("B".to_string()).append("A".to_string()).append("C".to_string()))
        .insert(LinkList::<String>::new().append("D".to_string()).append("B".to_string()).append("C".to_string()).append("A".to_string()))
        .insert(LinkList::<String>::new().append("D".to_string()).append("C".to_string()).append("A".to_string()).append("B".to_string()))
        .insert(LinkList::<String>::new().append("D".to_string()).append("C".to_string()).append("B".to_string()).append("A".to_string()));

        let mut permutation = permutations.clone();
        while !permutation.is_empty() {
            let first = values.find(&permutation.head().unwrap());
            assert!(!first.is_empty());

            let second = first.tail().unwrap().find(&permutation.head().unwrap());
            assert!(second.is_empty());

            permutation = permutation.tail().unwrap();
        }
    }

    #[test]
    fn test_map() {
        let list = LinkList::<i32>::new().append(1).append(2).append(3); // 1,2,3

        let mapped_list = list.map::<String>(|x| format!("{}", x + x));  // "2","4","6"

        assert_eq!("2".to_string(), mapped_list.head().unwrap());
        assert_eq!("4".to_string(), mapped_list.tail().unwrap().head().unwrap());
        assert_eq!("6".to_string(), mapped_list.tail().unwrap().tail().unwrap().head().unwrap());

        let mapped_list = LinkList::<i32>::new().map::<String>(|x| format!("{}", x + x));
        assert_eq!(LinkList::<String>::new(), mapped_list);
    }

    #[test]
    fn test_filter() {
        let list = LinkList::<i32>::new().append(1).append(2).append(3);

        // filter out the even values, leaving the odd
        let filtered = list.filter(|i| 1 == i % 2);
        assert!(2 == filtered.len());      // "filtered list has two elements",
        assert!(1 == filtered.head().unwrap());        // "first item in filtered list is 1",
        assert!(3 == filtered.tail().unwrap().head().unwrap());   // "second item in filtered list is 3",

        // filter out the odd values, leaving the event
        let filtered = list.filter(|i| 0 == i % 2);
        assert!(1 == filtered.len());  // "filtered list has two elements",
        assert!(2 == filtered.head().unwrap());    // "first item in filtered list is 2",

        // empty list yields empty list
        let list = LinkList::<i32>::new();
        let filtered = list.filter(|i| 0 == i % 2);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_fatten() {
        let list = LinkList::<i32>::new().append(1).append(2).append(3);
        let fattened = list.fatten();
        assert_eq!(
            fattened,
            LinkList::<LinkList<i32>>::new()
                .append(LinkList::<i32>::new().insert(1))
                .append(LinkList::<i32>::new().insert(2))
                .append(LinkList::<i32>::new().insert(3))
        );

        assert_eq!(LinkList::<i32>::new().fatten(), LinkList::<LinkList<i32>>::new());
    }

    #[test]
    fn test_flatten() {
        let list = LinkList::<LinkList<i32>>::new()
            .append(LinkList::<i32>::new().insert(1))
            .append(LinkList::<i32>::new().append(2).append(3))
            .append(LinkList::<i32>::new().insert(4));
        let flattened = list.flatten();
        assert_eq!(
            flattened,
            LinkList::<i32>::new().append(1).append(2).append(3).append(4)
        );

        assert_eq!(LinkList::<i32>::new(), LinkList::<LinkList<i32>>::new().flatten());
    }

    #[test]
    fn test_flatmap() {
        let list = LinkList::<LinkList<i32>>::new()
            .append(LinkList::<i32>::new().insert(1))
            .append(LinkList::<i32>::new().append(2).append(3))
            .append(LinkList::<i32>::new().insert(4));
        let flat_mapped = list.flatmap(|x| format!("{}", x + x));

        assert_eq!(
            flat_mapped,
            LinkList::<String>::new()
                .append("2".to_string())
                .append("4".to_string())
                .append("6".to_string())
                .append("8".to_string())
        );

        assert_eq!(LinkList::<String>::new(), LinkList::<LinkList<i32>>::new().flatmap(|x| format!("{}", x + x)));
    }

    #[test]
    fn test_combine() {
        let accumulator = LinkList::<LinkList<char>>::of_two(
            LinkList::<char>::of_three('a', 'b', 'c'),
            LinkList::<char>::of_three('d', 'e', 'f'));
        let list = LinkList::<char>::of_two('1', '2');

        let combinations: LinkList<LinkList<char>> = accumulator.combine(list);

        // println!("{:#?}", combinations);

        let correct = LinkList::<LinkList<char>>::of_four(
            LinkList::<char>::of_four('a', 'b', 'c', '1'),
            LinkList::<char>::of_four('a', 'b', 'c', '2'),
            LinkList::<char>::of_four('d', 'e', 'f', '1'),
            LinkList::<char>::of_four('d', 'e', 'f', '2'),
        );

        assert_eq!(4, combinations.len()); // "There should be 4 combinations.",
        let mut combinations = combinations.clone();
        while !combinations.is_empty() {
            let combination = combinations.head().unwrap();
            assert!(!correct.find(&combination).is_empty()); // "Each combination should be in the output",
            combinations = combinations.tail().unwrap();
        }
    }

    #[test]
    fn test_combinations() {
        // so combinations([[a, b], [c, d], [e, f]]) produces 2^3 = 8 combinations
        // [[a, c, e], [a, c, f], [a, d, e], [a, d, f], [b, c, e], [b, c, f], [b, d, e] [b, d, f]]

        let lists = LinkList::<LinkList<char>>::of_three(
            LinkList::<char>::of_two('a', 'b'), LinkList::<char>::of_two('c', 'd'), LinkList::<char>::of_two('e', 'f'));

        let combinations = lists.combinations();

        // println!("{:#?}", combinations);

        let correct = LinkList::<LinkList<char>>::of_eight(
            LinkList::<char>::of_three('a', 'c', 'e'),
            LinkList::<char>::of_three('a', 'c', 'f'),
            LinkList::<char>::of_three('a', 'd', 'e'),
            LinkList::<char>::of_three('a', 'd', 'f'),
            LinkList::<char>::of_three('b', 'c', 'e'),
            LinkList::<char>::of_three('b', 'c', 'f'),
            LinkList::<char>::of_three('b', 'd', 'e'),
            LinkList::<char>::of_three('b', 'd', 'f'),
        );

        assert_eq!(8, combinations.len()); // "There should be 8 combinations.",
        let mut combinations = combinations.clone();
        while !combinations.is_empty() {
            let combination = combinations.head().unwrap();
            assert!(!correct.find(&combination).is_empty()); // "Each combination should be in the output",
            combinations = combinations.tail().unwrap();
        }

    }
}
