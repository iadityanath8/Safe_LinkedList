# Linked List Implementation in Pure Safe Rust
# DO NOT USE THIS LINKED LIST IN PRODUCTION USE THE LINKED LIST IN STD LIBRARY OF RUST.
This Rust code defines a basic singly linked list using `Rc` (reference counting) and `RefCell` for interior mutability. The linked list is generic and can hold elements of any type that implements the `Debug` trait.

## Structures

### `Node<T>`
- Represents a node in the linked list.
- Contains generic data of type `T` and an optional reference to the next node (`Link<T>`).

### `List<T>`
- Represents the linked list itself.
- Consists of a head and a tail, both of type `Link<T>`.

## Methods

### `Node<T>::new(val: T) -> Self`
- Creates a new node with the given value.

### `List<T>::new() -> Self`
- Creates a new empty linked list.

### `List<T>::insert(val: T)`
- Inserts a new element at the end of the linked list.

### `List<T>::push(val: T)`
- Pushes a new element to the front of the linked list.

### `List<T>::pop()`
- Removes the first element from the linked list.

### `List<T>::debug_print()`
- Prints the elements of the linked list for debugging purposes.

## Testing

The code includes a test function `main2()` that demonstrates the usage of the linked list. Elements are pushed onto the list, and one element is popped. The resulting list is then printed for verification.

## How to Use

1. Create a new `List` using `List::<T>::new()`.
2. Insert elements using `insert()` or push elements to the front using `push()`.
3. Remove elements from the front using `pop()`.
4. Use `debug_print()` for debugging and verifying the contents of the linked list.

## Example

```rust
fn main() {
    let mut a = List::<i32>::new();
    a.push(89);
    a.push(891);
    a.push(10);
    a.push(1001);
    a.pop();

    a.debug_print();
}
