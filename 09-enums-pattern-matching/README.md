# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 09
> **Topic No.:** 09
> **Topic Name:** Enums & Pattern Matching
> **ประเด็นหลักที่ควรครอบคลุม:** enum, variants, pattern matching, match, if let

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายวุฒิชัย หลักเพชร | 670710142 | @670710142 | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวศุภิสรา สอนดี | 670710143 | @670710143 | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายจารุเดช พินิตศักดา | 670710144 | `@[กรอก GitHub username]` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นางสาวชัชชา เภาเสน | 670710145 | @670710145 | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`Enums (Enumerations) และ Pattern Matching เป็นฟีเจอร์ที่เป็นหัวใจสำคัญของการออกแบบภาษา Rust ในมุมมองของวิชาการภาษาโปรแกรม (Principles of Programming Languages) Enums ใน Rust ไม่ได้เป็นเพียงการตั้งชื่อให้ตัวเลข (Integer Constants) แบบในภาษา C/C++ หรือ Java แต่เป็น Algebraic Data Types (Sum Types) ซึ่งหมายความว่า Enum หนึ่งตัวสามารถบรรจุข้อมูลที่มีชนิด (Type) แตกต่างกันไว้ภายในได้`                                             `        เมื่อนำมารวมกับ Pattern Matching (match) ซึ่งเป็นกลไก Control Flow ที่ทรงพลัง Rust จะบังคับให้โปรแกรมเมอร์ต้องจัดการกับ "ทุกความเป็นไปได้ (Exhaustive checking)" เสมอ แนวคิดนี้ถูกนำมาใช้แก้ปัญหาที่ร้ายแรงที่สุดในโลกของการเขียนโปรแกรม เช่น การอ้างอิงค่าว่าง (Null Pointer Dereference) และการลืมดักจับ Error (Unhandled Exceptions) ทำให้โค้ดของ Rust มีความปลอดภัย (Type-safe) และคาดเดาพฤติกรรมได้สูงมาก`

---

## 4. Key Concepts

### 4.1 `Enums (Enumerations) หรือ Data Variants`

**คำอธิบาย**

`Enum ใน Rust ใช้สร้าง Custom Type ที่ค่าของมันสามารถเป็นไปได้เพียง "รูปแบบใดรูปแบบหนึ่ง" จากที่กำหนดไว้ (Variants) จุดเด่นคือ แต่ละ Variant ไม่จำเป็นต้องหน้าตาเหมือนกัน มันสามารถเป็นได้ทั้งแบบไม่มีข้อมูล (Unit-like), แบบเก็บข้อมูลเป็น Tuple, หรือแบบระบุชื่อฟิลด์ (Struct-like)`

**ตัวอย่าง**

```rust
// การสร้าง Enum ที่เก็บข้อมูลได้หลายรูปแบบ
enum Message {
    Quit,                       // Unit-like: ไม่มีข้อมูลข้างใน
    Move { x: i32, y: i32 },    // Struct-like: เก็บพิกัด x, y
    Write(String),              // Tuple-like: เก็บข้อความ
}

fn main() {
    let msg1 = Message::Write(String::from("Hello PPL"));
    let msg2 = Message::Move { x: 10, y: 20 };
}
```

**Explanation**

`จากโค้ด Message คือ Type เดียว แต่สามารถบรรจุข้อมูลที่แตกต่างกันโดยสิ้นเชิงได้ ทำให้เราสามารถจัดกลุ่มข้อมูลที่เกี่ยวข้องกันไว้ภายใต้ร่มเดียวกันได้อย่างเป็นระเบียบ`

---

### 4.2 `Pattern Matching (match)`

`match คือคำสั่งควบคุมทิศทางโปรแกรม (Control Flow) คล้ายกับ switch/case แต่ทรงพลังกว่าเพราะทำหน้าที่ Extract (ดึงข้อมูล) ที่ซ่อนอยู่ใน Enum ออกมาใช้งานได้ และมีกฎเหล็กคือ Exhaustiveness (ต้องเขียนครอบคลุมทุกกรณีที่ Enum เป็นไปได้ หากเขียนไม่ครบ Compiler จะแจ้ง Error ทันที)`

```rust
// Rust code
enum Coin {
    Penny,
    Quarter(String), // เก็บชื่อรัฐของเหรียญ Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        // Pattern Matching ดึงค่า String ออกมาใส่ตัวแปร state
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);
            25
        }
    }
}
```

**Explanation**

`match จะเปรียบเทียบค่า coin กับแต่ละ Pattern หากตรงกับ Quarter มันจะดึง String ที่อยู่ข้างในออกมาเก็บไว้ในตัวแปร state ให้เรานำไปปรินต์หรือประมวลผลต่อได้ทันที`


---

### 4.3 `การแทนที่ Null ด้วย Option<T>`

`Rust เป็นภาษาที่ไม่มีค่า Null (Null-safety) เพื่อป้องกันปัญหา Runtime Error แต่ Rust ใช้ Enum พิเศษที่มีอยู่ใน Standard Library ชื่อว่า Option<T> มาใช้แทน เพื่อสื่อถึงแนวคิดที่ว่า "อาจจะมีค่า (Some)" หรือ "ไม่มีค่า (None)"`

```rust
// Rust code
// Option Enum ถูก Build-in มาในภาษา หน้าตาเป็นแบบนี้:
// enum Option<T> { Some(T), None, }

fn main() {
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    match some_number {
        Some(val) => println!("มีตัวเลขคือ: {}", val),
        None => println!("ไม่มีข้อมูล (คล้าย Null แต่ปลอดภัย)"),
    }
}
```
**Explanation**

`เนื่องจาก Option<T> เป็น Enum การจะเอาค่า 5 ออกมาใช้บวกเลขตรงๆ จะทำไม่ได้ (Compiler จะด่า) โปรแกรมเมอร์ "ถูกบังคับ" ให้ต้องใช้ match แกะกล่อง Some และจัดการกรณี None เสมอ ทำให้ไม่มีโอกาสเกิด Null Pointer Exception`


---

### 4.4 `การจัดการ Error ด้วย Result<T, E>`

`Rust ไม่มีระบบ try/catch สำหรับ Exception Handling แบบภาษา OOP ทั่วไป แต่ใช้ Enum ที่ชื่อว่า Result<T, E> สำหรับฟังก์ชันที่อาจเกิดข้อผิดพลาดได้ โดยจะคืนค่า Ok(ข้อมูล) หากสำเร็จ และคืนค่า Err(ข้อผิดพลาด) หากล้มเหลว`

```rust
// Rust code
// Result Enum ถูก Build-in มาในภาษา หน้าตาเป็นแบบนี้:
// enum Result<T, E> { Ok(T), Err(E), }

fn main() {
    // การแปลง String เป็นตัวเลข อาจเกิด Error ได้
    let parse_result: Result<i32, _> = "100a".parse(); 

    match parse_result {
        Ok(number) => println!("แปลงสำเร็จ ได้เลข: {}", number),
        Err(e) => println!("แปลงไม่สำเร็จ เกิดข้อผิดพลาด: {}", e),
    }
}
```
**Explanation**

`การออกแบบนี้ (Return as a value) ทำให้ Error ถูกนำเสนอในรูปแบบของ Type อย่างชัดเจน ฟังก์ชันที่คืนค่า Result เป็นการประกาศให้ผู้เรียกใช้งานรู้ว่า "ฟังก์ชันนี้พังได้นะ ต้องจัดการ Error ด้วย"`

---

### 4.5 `Concise Control Flow ด้วย if let`

`ในบางครั้ง Enum มีหลายกรณี แต่เราสนใจแค่ "กรณีเดียว" การเขียน match ดักทุกทางอาจทำให้โค้ดยาวเกินไป Rust จึงให้ Syntax Sugar ที่ชื่อว่า if let มาเพื่อลดรูปการทำ Pattern Matching ในกรณีที่เราสนใจแค่ Pattern เดียว และปล่อยผ่าน (Ignore) กรณีอื่นๆ`

```rust
// Rust code
fn main() {
    let config_max = Some(3u8);

    // แบบที่ 1: ใช้ match (ต้องเขียน _ => () เพื่อดักกรณีที่เหลือ)
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // แบบที่ 2: ใช้ if let (กระชับกว่า อ่านง่ายกว่า)
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```
**Explanation**

`if let เป็นเพียงตัวย่อของ match ที่มีแค่ขาเดียว ช่วยลดความซ้ำซ้อนของโค้ด (Boilerplate) แต่ยังคงคุณสมบัติความปลอดภัยในการแกะกล่อง (Some) เช่นเดิม`
---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `enum Name { ... }` | `การประกาศสร้าง Custom Type ที่มีได้หลายรูปแบบ (Variants) โดยแต่ละแบบสามารถเก็บข้อมูลต่างชนิดกันได้` | `enum Status { Ok, Err(String) }` |
| `match value { ... }` | `คำสั่งสำหรับตรวจสอบและแยกแยะ Enum คล้าย switch/case แต่สามารถดึงข้อมูลที่อยู่ข้างในออกมาได้` | `match status { Status::Ok => ... }` |
| `_ => ...` | `Catch-all Pattern (Wildcard): ใช้ใน match เพื่อจัดการ "กรณีที่เหลือทั้งหมด" (เหมือน default ใน switch)` | `_ => println!("Ignore others"),` |
| `if let Pattern = value` | `การทำ Pattern Matching แบบสั้น (Syntax Sugar) ใช้เมื่อเราต้องการดึงข้อมูลและจัดการแค่ เงื่อนไขเดียว` | `if let Some(x) = option_val { ... }` |
| `Option<T> / Result<T, E>` | `Enum มาตรฐานของ Rust Option ใช้แทนค่า Null และ Result ใช้สำหรับจัดการ Error` | `let data: Option<i32> = Some(5);` |

### Important Rules

1. `Exhaustive Matching (ต้องเช็คให้ครบทุกกรณี): กฎเหล็กของภาษา Rust คือคำสั่ง match จะต้องครอบคลุม ทุกความเป็นไปได้ ของ Enum นั้นเสมอ หากเขียนดักไว้ไม่ครบ (และไม่ใช้ _) Compiler จะแจ้ง Error ทันที ทำให้เราไม่พลาดลืมเช็คเงื่อนไขใดเงื่อนไขหนึ่ง`
2. `No Direct Data Access (ห้ามเข้าถึงข้อมูลข้างในตรงๆ): คุณไม่สามารถเข้าถึงข้อมูลที่บรรจุอยู่ใน Enum ได้โดยตรง (เช่น message.text หรือ coin.state ทำไม่ได้) คุณ ต้อง ใช้ Pattern Matching (ผ่าน match หรือ if let) เพื่อทำหน้าที่ "แกะกล่อง" (Extract) และดึงข้อมูลออกมาสู่ Scope ปัจจุบันเสมอ`
3. `Top-to-Bottom Evaluation (ประเมินจากบนลงล่าง): Pattern ใน match จะถูกตรวจสอบจากบนลงล่างทีละบรรทัด เมื่อเจอ Pattern แรกที่ตรงกัน โปรแกรมจะทำงานใน Block นั้นแล้วออกจาก match ทันที ดังนั้นหากใช้ _ (Catch-all) จะต้องวางไว้ล่างสุดเสมอ`


---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `Non-exhaustive match`

**Problem**

`match ใน Rust ต้อง exhaustive ครอบคลุมทุก variant ของ enum ถ้าเขียนไม่ครบ โปรแกรมจะ compile ไม่ผ่าน`

**Incorrect Code**

```rust
enum Payment {
    Cash,
    CreditCard,
    PromptPay,
}

fn main() {
    let payment = Payment::Cash;

    match payment {
        Payment::Cash => println!("ชำระด้วยเงินสด"),
    }
}
```

**Correct Code**

```rust
enum Payment {
    Cash,
    CreditCard,
    PromptPay,
}

fn main() {
    let payment = Payment::Cash;

    match payment {
        Payment::Cash => println!("ชำระด้วยเงินสด"),
        _ => println!("ชำระด้วยวิธีอื่น"),
    }
}
```

**Why?**

`Payment มี variants 3 อัน คือ Cash , CreditCard , PromptPay แต่ match จัดการเพียง Cash จึงทำให้เกิด error non-exhaustive เพราะ match ใน Rust ต้อง exhaustive ครอบคลุมทุก variant ของ enum`
`ใน Correct Code เราใช้ _ เพื่อครอบคลุมกรณีที่เหลือให้แสดงผลออกมาเป็น "ชำระด้วยวิธีอื่น"`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[GitHub repository URL]`

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group XX]`

**Date:** `[YYYY-MM-DD]`
