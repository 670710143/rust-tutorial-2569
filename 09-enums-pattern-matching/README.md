# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 09
> **Topic No.:** 09
> **Topic Name:** Enums & Pattern Matching
> **ประเด็นหลักที่ควรครอบคลุม:** enum, variants, pattern matching, match, if let

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายวุฒิชัย หลักเพชร | 670710142 | `@[กรอก GitHub username]` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวศุภิสรา สอนดี | 670710143 | @670710143 | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายจารุเดช พินิตศักดา | 670710144 | `@670710144` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
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

`[เขียนเนื้อหาที่นี่ — ใช้โครงสร้างเดียวกับ rust_tutorial_template.md ฉบับเต็มที่ผู้สอนแจกให้]`

---

## 4. Key Concepts

### 4.1 `[Concept 1]`

**คำอธิบาย**

`[อธิบายแนวคิด]`

**ตัวอย่าง**

```rust
fn main() {
    println!("Hello, Rust!");
}
```

**Explanation**

`[อธิบายว่า code ทำงานอย่างไร]`

---

### 4.2 `[Concept 2]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.3 `[Concept 3]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.4 `[Concept 4 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.5 `[Concept 5 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |

### Important Rules

1. `[กฎสำคัญข้อที่ 1]`
2. `[กฎสำคัญข้อที่ 2]`
3. `[กฎสำคัญข้อที่ 3]`

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

`match ใน Rust ต้องครอบคลุมทุก variant ของ enum ถ้าเขียนไม่ครบ โปรแกรมจะ compile ไม่ผ่าน`

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

`Payment มี variants 3 อัน คือ Cash , CreditCard , PromptPay แต่ match จัดการเพียง Cash จึงทำให้เกิด error non-exhaustive เพราะ match ใน Rust ต้องครอบคลุมทุก variant ของ enum ใน Correct Code เราจึงใช้ _ เพื่อครอบคลุมทุกกรณีที่เหลือให้แสดงผลออกมาเป็น "ชำระด้วยวิธีอื่น"`

---

### Mistake 2 — `สับสนเรื่อง Ownership เมื่อใช้ match`

**Problem**

`ใช้ match value แล้วข้อมูลอย่าง String อาจถูกย้ายความเป็นเจ้าของไปใน match ทำให้ไม่สามารถใช้ตัวแปรนั้นซ้ำได้`

**Incorrect Code**

```rust
enum Message {
    Write(String),
}
fn main() {
    let msg = Message::Write(String::from("have a nice day")) ;
    match msg {
        Message::Write(text) => println!("{}",text),
    }
    match msg {
        Message::Write(text) => println!("{}",text),
    }
}
```

**Correct Code**

```rust
enum Message {
    Write(String),
}
fn main() {
    let msg = Message::Write(String::from("have a nice day")) ;
    match &msg {
        Message::Write(text) => println!("{}",text),
    }
    match &msg {
        Message::Write(text) => println!("{}",text),
    }
}
```

**Why?**

`match msg จะเอาค่าที่อยู่ใน msg มาใช้ใน match ถ้าข้อมูลนั้นเป็น String อาจทำให้ค่าถูกย้ายออกไปแล้วทำให้ msg ใช้ต่อไม่ได้ แต่การใช้ match &msg จะเป็นการยืมข้อมูลมาดู ทำให้ยังสามารถใช้ msg ต่อได้หลังจาก match`

---

### Mistake 3 — `ลำดับ Pattern ใน match ผิด`

**Problem**

`การเขียน pattern ที่ครอบคลุมทุกกรณีไว้ก่อน pattern ที่เฉพาะเจาะจงกว่า ทำให้ pattern ที่อยู่ด้านหลังไม่สามารถทำงานได้`

**Incorrect Code**

```rust
enum Status {
    Success,
    Error,
    Pending,
}
fn show_status(status: Status) {
    match status {
        Status::Success => println!("สถานะสำเร็จ"),
        _ => println!("สถานะอื่น"),
        Status::Error => println!("เกิดข้อผิดพลาด"),
    }
}
fn main() {
    show_status(Status::Error);
}
```

**Correct Code**

```rust
enum Status {
    Success,
    Error,
    Pending,
}
fn show_status(status: Status) {
    match status {
        Status::Success => println!("สถานะสำเร็จ"),
        Status::Error => println!("เกิดข้อผิดพลาด"),
        _ => println!("สถานะอื่น"),
    }
}
fn main() {
    show_status(Status::Error);
}
```

**Why?**

`match จะตรวจสอบ pattern จากบนลงล่าง ใน Incorrect Code _ หมายถึงทุกกรณีที่เหลือ จึงทำให้ Status::Error ที่อยู่ด้านหลังไม่สามารถทำงานได้ ดังนั้นควรวาง pattern ที่เฉพาะเจาะจงไว้ก่อน แล้วค่อยใช้ _ สำหรับกรณีที่เหลือ`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `Calculate Shape Area`

**Problem**

`เขียน enum ชื่อ Shape มี variant 3 ตัว ได้แก่ Circle, Rectangle, Triangle จากนั้นเขียนฟังก์ชัน area ที่รับ Shape และคำนวณพื้นที่แต่ละรูปโดยใช้ match`

**Hint**

`พื้นที่วงกลม = π × r²`  
`พื้นที่สี่เหลี่ยม = กว้าง × ยาว`  
`พื้นที่สามเหลี่ยม = 0.5 × ฐาน × สูง`

**Solution**

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}
fn main() {
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle { base: 5.0, height: 6.0 };
    println!("Circle = {:.2}", area(&circle));
    println!("Rectangle = {:.2}", area(&rectangle));
    println!("Triangle = {:.2}", area(&triangle));
}
```

**Explanation**

`Shape ใช้เก็บข้อมูลของรูปแต่ละแบบ ฟังก์ชัน area ใช้ match เพื่อเช็คว่าเป็นรูปอะไรแล้วคำนวณหาพื้นที่ตามข้อมูลที่เก็บไว้ในแต่ละ variant โดยฟังก์ชันรับ &Shape เพื่อยืมค่าทำให้สามารถนำ Shape ไปใช้ต่อได้ variant ที่ใช้วงเล็บ() จะเก็บข้อมูลตามลำดับ ส่วน variant ที่ใช้ปีกกา{} จะเก็บข้อมูลโดยระบุชื่อ field`

---

### Exercise 2 — `Message Handler`

**Problem**

`เขียน enum ชื่อ Message มี 3 variants ได้แก่ Quit, Move, Write จากนั้นเขียนฟังก์ชัน handle_message ที่ใช้ match เพื่อตรวจสอบชนิดของ message และแสดงผลลัพธ์ที่เหมาะสม`

**Hint**

`Quit -> แสดง "Quit"`  
`Move -> แสดงตำแหน่ง x , y`  
`Write -> แสดงข้อความที่ส่งมา`

**Solution**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
fn handle_message(message: &Message) {
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Position: {}, {}", x, y),
        Message::Write(text) => println!("Message: {}", text),
    }
}
fn main() {
    let quit = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("Have a nice day"));
    handle_message(&quit);
    handle_message(&move_message);
    handle_message(&write);
}
```

**Explanation**

`Message ใช้เก็บข้อมูลของข้อความแต่ละประเภท โดย Quit ไม่มีข้อมูลเพิ่มเติม Move เก็บตำแหน่ง x กับ y และ Write เก็บข้อความเป็น String ฟังก์ชัน handle_message ใช้ match เพื่อตรวจสอบว่า Message เป็นแบบไหน แล้วดึงข้อมูลที่อยู่ในแต่ละ variant ออกมาแสดง`

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
| Syntax | `เขียนสั้น ชัดเจน รวมการสร้างตัวแปรและแกะค่าในตัวเดียว ด้วยmatch หรือ if let` | `syntaxรก ถ้าจะเก็บค่าหลายแบบต้องใช้ std::visit กับ std::variant` |
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
