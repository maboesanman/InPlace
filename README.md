# InPlace

[documentation](https://maboesanman.github.io/InPlace/in_place)

Experimental design for a trait based approach to the `Entry` types

the design specifically considers the following data structures:
* `HashMap`
* `BTreeMap`
* `HashSet`
* `BTreeSet`
* `LinkedList`*
* `Vec`*
* `VecDeque`*

the real value of the design is really motivated by HashMap and BTreeMap.

this can also be applied to collections from outside the standard library.

There are two main parts: entry and get_entry.

Entries represent some in-progress collection operation, such as insert. The underlying collection must always be valid, but you've bookmarked whatever information you've learned so it doesn't have to be verified again.

for example, `get_occupied(usize)` on `[T]` returns an occupied entry if the bounds checks pass. subsequent calls to get references to the value in the entry do not perform bounds checks because they must still be good.
