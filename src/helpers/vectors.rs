use crate::helpers::iterators::skip_iterator;

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
pub fn permutations<T>(list: Vec<T>) -> Vec<Vec<T>> where T: Copy
{
    let mut results = Vec::<Vec<T>>::default();

    if list.len() == 1 {
        // single element list has one permutation
        results.push(vec!(list[0]))
    } else if list.len() > 1 {
        //
        // 1. i = i;
        // 2. left = ith element, right = list without ith element;
        // 2. get all permutations of the right
        // 3. combine left + right permutations
        // 4. combine right permutations + left.
        // 6. i += 1
        //
        let mut i = 0;
        for left in &list
        {
            let mut j = 0;
            for right in &list
            {
                if j != i {
                    results.push(vec!(*left, *right))
                }
                j += 1;
            }

            i += 1;
        }
    }
    results
}

/**
 * Create all combinations of the elements of two lists and
 * return the set of combinations
 *
 * so combine([a, b], [c, d], [e, f]) produces 2^3 = 8 combinations
 * [[a, c, e], [a, c, f], [a, d, e], [a, d, f], [b, c, e], [b, c, f], [b, d, e] [b, d, f]]
 *
 * @param <T>
 * @return
 */
pub fn combinations<T>(list: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Copy
{
    //
    // private recursive helper that takes an accumulator
    //
    fn inner_combine<T>(list: Vec<Vec<T>>, mut accumulator: Vec<Vec<T>>)  -> Vec<Vec<T>>  where T: Copy
    {
        if list.len() == 0 {
            // list is empty
            accumulator
        } else {
            // we have at least two lists to combine
            // return innerCombine(list.tail, combineAppend(accumulator, list.head));
            // inner_combine(Vec::from_iter(skip_iterator(list, 0, 1)), combine_append(accumulator, list[0]))
            inner_combine(Vec::from(list[1..]), combine_append(accumulator, list[0]))
        }
    }

    return inner_combine(list, Vec::<Vec<T>>::new());
}


/**
 * Create all combinations of appending the list elements to
 * the lists in the accumulator.
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
pub fn combine_append<T>(accumulator: Vec<Vec<T>>, list: Vec<T>) -> Vec<Vec<T>>
{
    let mut iter = list.into_iter();
    if let Some(item) = iter.next() {
        if accumulator.len() == 0
        {
            //
            // nothing in accumulator,
            // so result is just the elements of list
            //
            return deepen(list);
        }

        //
        // generate all the combinations of appending elements of list
        // to the lists in accumulator.
        //
        let mut combinations = Vec::<Vec<T>>::new();
        //for(LinkList<LinkList<T>> a = accumulator; a.isNotEmpty(); a = a.tail)
        for a in accumulator
        {
            // for(LinkList<T> l = list; l.isNotEmpty(); l = l.tail)
            for l in list
            {
                // combinations = combinations.insert(a.head.append(l.head));
                let mut combined = Vec::<T>::from(a);
                combined.push(l);
                combinations.insert(0, combined);   // insert at start of list
            }
        }

        return combinations;
    } else {
        // list is empty
        return accumulator;
    }

}

/**
 * Convert an iterable into a list of lists by element.
 *
 * fatten([1, 2, 3]) yields [[1], [2], [3]]
 *
 * @param list
 * @param <T>
 * @return
 */
pub fn deepen<T>(list: impl IntoIterator<Item = T>) -> Vec<Vec<T>>
{
    list.into_iter().map(|item| vec!(item)).collect()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_permutations()
    {
        let list = vec!("A", "B", "C", "D");

        let permutations = permutations(list);

        //for(LinkList<LinkList<String>> permutation = permutations; permutation.isNotEmpty(); permutation = permutation.tail)
        for permutation in permutations
        {
            println!("{:?}", permutation);
        }

        assert_eq!(24, permutations.iter().count(), "there should be 24 permutations");

        let values = vec!(
            vec!("A","B","C","D"),
            vec!("A","B","D","C"),
            vec!("A","C","B","D"),
            vec!("A","C","D","B"),
            vec!("A","D","B","C"),
            vec!("A","D","C","B"),

            vec!("B","A","C","D"),
            vec!("B","A","D","C"),
            vec!("B","C","A","D"),
            vec!("B","C","D","A"),
            vec!("B","D","A","C"),
            vec!("B","D","C","A"),

            vec!("C","A","B","D"),
            vec!("C","A","D","B"),
            vec!("C","B","A","D"),
            vec!("C","B","D","A"),
            vec!("C","D","A","B"),
            vec!("C","D","B","A"),

            vec!("D","A","B","C"),
            vec!("D","A","C","B"),
            vec!("D","B","A","C"),
            vec!("D","B","C","A"),
            vec!("D","C","A","B"),
            vec!("D","C","B","A"),
        );

        //for(LinkList<LinkList<String>> permutation = permutations; permutation.isNotEmpty(); permutation = permutation.tail)
        for permutation in permutations
        {
            let n = values.iter().filter(|&&item| item == permutation).count();
            assert_eq!(1, n, "There should be exactly one entry in values for each permutation.");
        }
    }

    // unit test for combinations()
    #[test]
    fn test_combinations()
    {
        let list = vec!(vec!("a", "b"), vec!("c", "d"), vec!("e", "f"));
        let combinations = combinations(list);
        println!("{:?}", combinations);
    }

    // @Test
    // public void testCombineAppend()
    // {
    //     final LinkList<LinkList<Character>> accumulator = LinkLists.linkList(
    //         LinkLists.linkList('a', 'b', 'c'), LinkLists.linkList('d', 'e', 'f'));
    //     final LinkList<Character> list = LinkLists.linkList('1', '2');

    //     final LinkList<LinkList<Character>> combinations = LinkLists.combineAppend(accumulator, list);

    //     for(LinkList<Character> combination : new IterableLinkList<>(combinations))
    //     {
    //         System.out.println(combination.toString());
    //     }

    //     final LinkList correct = LinkList.Nil
    //         .append("[a:b:c:1"
    //         .append("[a:b:c:2"
    //         .append("[d:e:f:1"
    //         .append("[d:e:f:2";

    //     assertEquals("There should be 4 combinations.", 4, combinations.size());
    //     for(LinkList<Character> combination : new IterableLinkList<>(combinations))
    //     {
    //         assertTrue("Each combination should be in the output", correct.find(combination.toString()).isNotEmpty());
    //     }
    // }

    // @Test
    // public void testCombineElements()
    // {
    //     // so combine([a, b], [c, d], [e, f]) produces 2^3 = 8 combinations
    //     // [[a, c, e], [a, c, f], [a, d, e], [a, d, f], [b, c, e], [b, c, f], [b, d, e] [b, d, f]]

    //     final LinkList<LinkList<Character>> lists = LinkLists.linkList(
    //         LinkLists.linkList('a', 'b'), LinkLists.linkList('c', 'd'), LinkLists.linkList('e', 'f'));

    //     final LinkList<LinkList<Character>> combinations = LinkLists.combinations(lists);

    //     for(LinkList<Character> combination : new IterableLinkList<>(combinations))
    //     {
    //         System.out.println(combination.toString());
    //     }

    //     final LinkList correct = LinkList.Nil
    //         .append("[a:c:e"
    //         .append("[a:c:f"
    //         .append("[a:d:e"
    //         .append("[a:d:f"
    //         .append("[b:c:e"
    //         .append("[b:c:f"
    //         .append("[b:d:e"
    //         .append("[b:d:f";

    //     assertEquals("There should be 8 combinations.", 8, combinations.size());
    //     for(LinkList<Character> combination : new IterableLinkList<>(combinations))
    //     {
    //         assertTrue("Each combination should be in the output", correct.find(combination.toString()).isNotEmpty());
    //     }


    // }
}
