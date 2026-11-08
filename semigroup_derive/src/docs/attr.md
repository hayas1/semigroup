| attribute | content |
| --- | --- |
| `bound = ".."` | where predicates for every impl, such as `bound = "T: Default, T: Copy"` |
| `bound(..)` | where predicates for each impl: `semigroup`, `annotated`, `selective`, `monoid`, `commutative` |
| `crate = ..` | the path of the `semigroup` crate, `::semigroup` unless given |

As in serde, a bound *replaces* the bounds the derive would infer for that impl,
and `bound` takes one of its two forms. `bound = ""` opts out of the inferred bounds.
