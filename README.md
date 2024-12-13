# Yttrium - a yaml validator written in Rust

This validator is a PoC to test how state machine can be relevant when it comes to YAML parsing/validation. **It is not prod ready** since it only implements a few rules of the yaml grammar.

## Rules

### Scalar

Scalars are basically a sequence of characters : a-Z, A-Z and space.

### Key values

Key values are a line that matches the following expression :

```
<scalar> : <scalar>
```

Where the first scalar is the key and the second scalar is a value.

### Lists

List are useful to associate a set of values to a key. They match the following expression :

```
<scalar>:
- <scalar>
- <scalar>
- <scalar>
- <scalar>
```
