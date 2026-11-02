# GitHub Actions CI/CD | 공통 ARM64 기반과 산출물별 배포

이 프로젝트에서는 CI/CD를 **자동 빌드·패키징·배포**에 사용했습니다. 모든 테스트를 강제하는 품질 게이트와 무중단·자동 롤백을 구축했다고 주장하지 않습니다. 공개 검토 사본은 실행 비활성 상태이며 아래 설정을 읽을 수 있도록 보존했습니다.

## 1. 공통 빌드 → 서비스별 확장 → 실행 이미지

[공통 ARM64 Rust 툴체인·linker·sccache](../aws/base-aarch64.dockerfile#L1-L18) → [API 전용 선행 빌드층](../aws/base-aarch64-site-api.dockerfile#L1-L17) 등 3개 서비스별 기반 → [실행 이미지 분리](../site-api/dockerfile#L1-L55)로 구성했습니다. 웹은 Trunk·WASM 도구로 dist를 만듭니다.

## 2. 네 workflow의 차이

| workflow | 자동화한 경로 |
| --- | --- |
| [site-api.yml](../.github/workflows/site-api.yml#L42-L86) | QEMU/Buildx ARM64 → ECR → S3 배포 묶음 → CodeDeploy |
| [batch_exe.yml](../.github/workflows/batch_exe.yml#L1-L84) | 스케줄러 이미지와 배포 묶음 → CodeDeploy |
| [batch.yml](../.github/workflows/batch.yml#L1-L68) | 배치 ARM64 이미지 → ECR |
| [site.yml](../.github/workflows/site.yml#L1-L71) | Docker 안에서 Rust/WASM 웹 빌드 → dist 추출 → S3 |

경로별 변경 필터·수동 실행을 구성했습니다. batch·batch_exe·site-api는 latest/commit 태그를 발행하고 site는 latest 로컬 이미지에서 dist를 추출합니다. 캐시 정책은 API no-cache와 다른 경로의 캐시 설정으로 구분됩니다. 원본에는 네 workflow 모두 과거 성공 실행 기록이 있고, 해당 기록은 원본 초대 후 확인할 수 있습니다.

## 3. 배치용 EC2 기동·정지

당시 DB / API+스케줄러 / 배치 전용으로 EC2 3역할을 구성했습니다. 작업 조건·계획 ID를 user-data에 전달하고 기존 인스턴스 설정 변경 → 시작 → 부팅 후 배치 → 완료 API → 정지로 연결했습니다. 공개 사본의 계정·버킷·인스턴스·호스트 값은 예시로 바꿨습니다.
