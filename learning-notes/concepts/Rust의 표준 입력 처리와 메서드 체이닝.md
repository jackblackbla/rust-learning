## std::io::stdin().read_line()의 동작 원리

### 1. 기본 구조

```
let mut input = String::new();
let byte_count = std::io::stdin().read_line(&mut input).unwrap();
```

### 2. 동작 과정

1. `read_line(&mut input)`
    - 매개변수로 받은 `input`에 사용자 입력 문자열 저장
    - 반환값: `Result<usize, Error>` 타입
        - 성공: 읽은 바이트 수(usize)
        - 실패: Error
2. `unwrap()`
    - Result에서 성공한 값만 추출
    - 이 경우 읽은 바이트 수(usize) 반환
    - 실패 시 프로그램 중단

### 3. UTF-8 인코딩의 바이트 수

- ASCII 문자: 1바이트
- 한글: 3바이트
- 엔터키(\n): 1바이트

예시:
```
// "가나" + 엔터 입력 시
// -> 7바이트 ('가'(3) + '나'(3) + '\n'(1))
```

### 4. 메서드 체이닝 시 주의사항

```
// ❌ 잘못된 사용
let result = std::io::stdin().read_line(&mut input).unwrap().trim();
// unwrap()이 반환하는 것은 바이트 수(숫자)이므로 trim() 불가

// ✅ 올바른 사용 
std::io::stdin().read_line(&mut input).unwrap();
let result = input.trim();
```

### 5. 정리

- `read_line()`은 입력 문자열을 매개변수에 저장
- 반환값은 읽은 바이트 수
- 문자열 처리(trim 등)는 저장된 문자열 변수에 대해 수행해야 함
