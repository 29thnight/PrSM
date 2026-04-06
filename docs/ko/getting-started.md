---
title: 시작하기
parent: 시작하기 (KO)
nav_order: 2
---

# 시작하기

## 설치

### 방법 A: MSI 인스톨러 (Windows, 권장)

[GitHub Releases](https://github.com/29thnight/PrSM/releases)에서 최신 `.msi`를 다운로드하여 실행합니다. 인스톨러가 수행하는 작업:
- `prism.exe`를 `C:\Program Files\PrSM\`에 설치
- 시스템 PATH에 설치 디렉토리 추가
- VS Code가 감지되면 확장 자동 설치
- 설치 완료 시 Unity 패키지 설치 안내 표시

### 방법 B: winget (Windows)

```powershell
winget install PrSM.PrSM
```

### 방법 C: 소스 빌드

사전 요구사항:
- [Rust 툴체인](https://rustup.rs/) (stable)
- Node.js 20+ 및 npm (VS Code 확장용)

```powershell
cargo build --release -p refraction
```

컴파일러 바이너리: `target/release/prism.exe` (Windows) 또는 `target/release/prism` (macOS/Linux).

VS Code 확장:

```powershell
cd vscode-prsm
npm install
npm run bundle    # 확장 번들링 + prism 바이너리 복사
npm run package   # .vsix 생성
```

VS Code에서 **확장 > VSIX에서 설치**로 설치합니다.

## 첫 걸음

### 1. 설치 확인

```powershell
prism version
```

예상 출력: `prism 0.1.0` (또는 유사).

### 2. 프로젝트 초기화

```powershell
cd MyUnityProject
prism init
```

`.prsmproject` 파일이 생성됩니다. 모든 옵션은 [프로젝트 설정](project-configuration.md)을 참조하세요.

### 3. 첫 번째 컴포넌트 작성

`Assets/Player.prsm` 생성:

```prsm
using UnityEngine

component Player : MonoBehaviour {
    serialize speed: Float = 5.0

    require rb: Rigidbody

    update {
        val h = input.axis("Horizontal")
        val v = input.axis("Vertical")
        rb.velocity = vec3(h, 0, v) * speed
    }
}
```

### 4. 컴파일

```powershell
prism build
```

설정된 출력 디렉토리에 `Player.cs`와 `Player.prsmmap.json`이 생성됩니다.

### 5. Unity에서 사용

Unity 프로젝트를 엽니다. 생성된 C#은 Unity가 자동으로 컴파일합니다. Rigidbody가 있는 GameObject에 `Player` 컴포넌트를 추가합니다.

## Unity 패키지 설치

Package Manager를 통해 PrSM Unity 패키지를 추가합니다:

1. **Window > Package Manager** 열기
2. **+** > **Add package from git URL** 클릭
3. 입력: `https://github.com/29thnight/PrSM.git?path=unity-package`

패키지가 제공하는 기능:
- `.prsm` 파일 자동 임포트 (ScriptedImporter)
- Compile/Check/Build 컨텍스트 메뉴
- PrSM 컴포넌트 커스텀 인스펙터
- 생성 C# → `.prsm` 소스 스택트레이스 리맵
- 드래그 앤 드롭 컴포넌트 추가

## Watch 모드

지속적 개발을 위해:

```powershell
prism build --watch
```

소스 디렉토리를 감시하며 변경된 파일을 자동으로 재컴파일합니다.

## 분석 명령

VS Code 확장의 탐색 기능을 구동하는 명령들:

```powershell
prism check Assets/Player.prsm              # 진단만
prism check Assets/Player.prsm --json       # 기계 판독 가능 출력
prism hir . --json                            # Typed HIR 덤프
prism definition . --json --file Player.prsm --line 10 --col 5
prism references . --json --file Player.prsm --line 10 --col 5
prism index . --json --symbol Player
```

## 트러블슈팅

### `prism` 명령을 찾을 수 없음

- **MSI 설치**: 설치 후 터미널을 재시작하여 PATH가 적용되게 합니다.
- **소스 빌드**: `target/release/`를 PATH에 추가하거나, `prism.exe`를 PATH에 있는 디렉토리에 복사합니다.

### VS Code 확장이 활성화되지 않음

- 워크스페이스가 **신뢰됨** 상태인지 확인합니다 (파일 > 작업 영역 신뢰 관리). LSP는 신뢰된 워크스페이스에서만 실행됩니다.
- 출력 패널 (**보기 > 출력 > PrSM Language Server**)에서 시작 오류를 확인합니다.
- 컴파일러 경로를 확인합니다: **설정 > prsm.compilerPath**가 유효한 `prism` 바이너리를 가리켜야 합니다.

### Unity에서 `.prsm` 파일을 감지하지 못함

- PrSM Unity 패키지가 설치되었는지 확인합니다 (Project 창에서 Packages 확인).
- Unity 프로젝트 루트에 `.prsmproject`가 존재하는지 확인합니다.
- 메뉴 바에서 **PrSM > Build Project**를 시도합니다.
