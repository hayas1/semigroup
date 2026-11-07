| attribute | content |
| --- | --- |
| `properties(..)` | the properties to derive besides `Semigroup`: `annotated(param = A)`, `selective`, `monoid(identity = <expr>)`, `commutative`; a bare word takes the defaults |
| `manual_impl(..)` | the impls to leave out so that they can be written by hand: `semigroup`, `selective`, `monoid` |
| `bound = ".."` | where predicates for every impl, such as `bound = "T: Default, T: Copy"` |
| `bound(..)` | where predicates for each impl: `semigroup`, `annotated`, `selective`, `monoid`, `commutative` |

As in serde, a bound *replaces* the bounds the derive would infer for that impl,
and `bound` takes one of its two forms.
`manual_impl(..)` and `bound(..)` may name only properties listed in `properties(..)`; `semigroup` is always derived.
