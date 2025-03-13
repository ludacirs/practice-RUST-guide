# Practice-RUST-Guide

러스트 프로그래밍 공식 가이드를 보고 공부하는 레포지토리

## 복습 포인트

### 3-1 변수와 가변성

```rust
    let x = 5; // 불변 변수 선언
    let mut x = 5; // 가변 변수 선언
```

```rust
    let mut x = 5;
    x = 6 // 변수 재할당 가능 ⭕
```

```rust
    let x = 5;
    x = 6 // 변수 재할당 불가능 ❌
```

```rust
    let mut x = 5;
    let x = 7 // **섀도잉**
```

```rust
    let x = 5;
    let x = 7 // **섀도잉**
```

- 섀도잉: 기존 변수를 무시하며 메모리를 공유하지않고 새로운 변수를 선언
- 이것 때문에 버그가 더 자주 발생할 것 같은데...
