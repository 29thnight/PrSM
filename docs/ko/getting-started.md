---
title: 시작하기
parent: 시작하기 (KO)
nav_order: 2
---

# 시작하기

## 사전 준비

- [Rust 툴체인](https://rustup.rs/) (stable)
- Node.js 18 이상 및 npm (VS Code 확장 빌드 시 필요)
- Unity 2021.3 LTS 이상 (Unity 패키지 사용 시 필요)
- VS Code + PrSM 확장 (선택 사항이지만 권장)

## 1. 컴파일러 빌드

```powershell
cargo build -p refraction
```

테스트로 빌드 상태를 확인할 수도 있습니다.

```powershell
cargo test
```

## 2. 샘플 파일 컴파일

```powershell
cargo run -p refraction --bin prism -- compile samples\PlayerController.prsm --output build-output
```

`build-output` 디렉토리에 생성된 `.cs` 파일과 `.prsmmap.json` 사이드카가 만들어집니다.

## 3. 출력 없이 검사

```powershell
cargo run -p refraction --bin prism -- check samples\PlayerController.prsm
```

`--json`을 붙이면 기계가 읽을 수 있는 진단 결과가 출력됩니다.

## 4. 새 프로젝트 초기화

```powershell
cargo run -p refraction --bin prism -- init
```

현재 디렉토리에 `.prsmproject` 파일을 생성합니다. 소스 glob 패턴, 출력 디렉토리, 대상 Unity 버전을 편집하여 설정합니다.

## 5. 전체 프로젝트 빌드

```powershell
cargo run -p refraction --bin prism -- build
```

개발 중 지속적 컴파일을 원한다면 `--watch`를 사용합니다.

```powershell
cargo run -p refraction --bin prism -- build --watch
```

## 6. 분석 명령 살펴보기

이 명령들은 VS Code 확장의 탐색 기능을 구동합니다.

```powershell
# Typed HIR 덤프
cargo run -p refraction --bin prism -- hir . --json

# 파일 위치에서 정의 해석
cargo run -p refraction --bin prism -- definition . --json --file samples\PlayerController.prsm --line 10 --col 5

# 위치의 심볼에 대한 모든 참조 찾기
cargo run -p refraction --bin prism -- references . --json --file samples\PlayerController.prsm --line 10 --col 5

# 이름으로 프로젝트 심볼 인덱스 조회
cargo run -p refraction --bin prism -- index . --json --symbol PlayerController
```

## 7. VS Code 확장

```powershell
cd vscode-prsm
npm install
npm test          # 확장 테스트 실행
npm run package   # .vsix 패키징
```

생성된 `.vsix`는 `vscode-prsm/artifacts/`에 위치하며, VS Code에서 **확장 > VSIX에서 설치**로 설치할 수 있습니다.
