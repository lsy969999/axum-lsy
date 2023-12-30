## 목차
1. 명명규칙
2. 도메인
3. 테이블
   1. 1.공통코드_tb_cmmn_code
---
## 명명규칙
|약어|풀네임|비고|
|-|-|-|
|tb|table|테이블|
|seq|sequence|시퀀스|
|idx|index|인덱스|
|cmm|common|공통|
|ty|type|유형|
|se|separator|구분|
|stt|status|상태|
|val|value|값|
|nm|name|명|
|grp|group|그룹|
|yn|yes or no|여부|
|hst|history|이력|
|dt|timestamp|일시|
|de|date|일자|
|dd||일자|
|desc|description|설명|
|sn|serial number|일련번호|
---
## 도메인
|도메인|이름|도메인타입|비고|
|-|-|-|-|
|-|-|-|자유|
|cmm_code_30|공통코드|varchar(30)|
|sort_3|정렬|numeric(3)|
---
## 테이블
### tb_cmm_code
|L|P|D|
|-|-|-|
|tb_cmm_code|공통코드테이블|공통코드테이블 |

|KEY|LOGICAL|PHYSICAL|TYPE|NULL|DEFAULT|INDEX|DOMAIN|REMARK|
|-|-|-|-|-|-|-|-|-|
|PK|코드아이디|code_id|varchar(30)|NULL|-|-|cmm_code_30|-|
|PK|코드값|code_val|varchar(30)|NULL|-|-|cmm_code_30|-|
|-|코드명|code_nm|varchar(30)|NULL|-|-|cmm_code_30|-|
|-|코드그룹|code_grp|varchar(30)|NULL|-|-|cmm_code_30|-|
|-|정렬|sort|numeric(3)|NULL|-|-|sort_3|-|
|-|사용여부|use_yn|varchar(1)|NOT NULL|Y|-|-|-|
|-|생성일시|created_dt|timestamp|NOT NULL|-|-|-|-|
|-|생성자|created_by|numeric()|NOT NULL|-|-|-|-|
|-|수정일시|updated_dt|timestamp|NOT NULL|-|-|-|-|
|-|수정자|updated_by|numeric()|NOT NULL|-|-|-|-|
```sql
--table generate

--sequence generate

--index generate
```
### tb_account
|KEY|LOGICAL|PHYSICAL|TYPE|NULL|DEFAULT|INDEX|REMARK|
|---|---|---|---|---|---|---|---|
|PK|-|-|-|-|-|-|-|

SEQUENCE: tb_account_seq
```sql
--table generate

--sequence generate

--index generate
```

### tb_user
```sql
--table generate

--sequence generate

--index generate
```
### tb_account_refresh_token
```sql
--table generate

--sequence generate

--index generate
```



