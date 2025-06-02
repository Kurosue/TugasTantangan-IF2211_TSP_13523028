# Tugas Tantangan IF2211 - TSP Solver
 
<div align="center">
  <img width="65%"src="https://github.com/user-attachments/assets/fbb348a6-cd98-46ec-930f-81c66ec59725" alt="App Preview(placeholder bini guweh)" />
  <br><strong> Firefly with me at last weekend after UAS :D( ofc, i took this photo)
</div>

 <div align="center">
   <h3 align="center">Languages</h3>
 
   <p align="center">

[![Rust](https://img.shields.io/badge/Rust-DEA258?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
 
   </p>
 </div>
 <div align="justify">  </div>
<br />

### Table of Contents
- [Overview](#overview)
- [Installation & Setup](#installation--setup)
  - [Requirements](#requirements)
  - [Installing Dependencies and Requirement](#installing-dependencies-and-requirement)
- [How to Run](#how-to-run)
- [Screenshot](#screenshot-program)
- [Author](#author)

---
### Overview
Travelling Salesman Problem (TSP) Solver menggunakan pendekatan Dynamic Programming dengan Bitmasking. Diberikan sejumlah kota beserta jarak antar setiap kota, program akan menentukan rute minimum yang dimulai dari kota pertama, mengunjungi setiap kota tepat satu kali, lalu kembali ke kota asal.

Program ini mengimplementasikan fungsi rekursif:

- State: `f(i,s)`
- Basis Rekursif: `f(i, ∅) = c[i][1]`
- Rekursif: `f(i, S) = min { c[i][j] + f(j, S - {j}) | j ∈ S }`

Program membaca input dari terminal dalam format berikut:
1. **Baris Pertama** menererima input integer `N` menyatakan banyak kota.
2. **N Baris selanjutnya** menyatakan _cost_ atau _weight_ atau jarak antar kota.
Contoh Input:
```
4
0 10 15 20
5 0 9 10
6 13 0 12
8 8 9 0
```
- Ini berarti ada 4 kota.
- Jarak dari kota 1 ke kota 2 adalah 10, ke kota 3 adalah 15, dan seterusnya.


<br />

 ---
 
 ## Installation & Setup
 
 ### Requirements
 > - Rust
 > - yeah, just rust

 <br/>

 ### Installing Dependencies and Requirement

<a id="dependencies"></a>
1. Install [Rust](https://www.rust-lang.org/tools/install)
    ```bash
    curl https://sh.rustup.rs -sSf | sh # Only Linux
    ```
<br>  
<br/>  

 ---
 ## How to Run
 1. Clone the repository
    ```   bash
    git clone https://github.com/Kurosue/TugasTantangan-IF2211_TSP_13523028.git
    ```
 2. Go to the project directory:
    ```bash
    cd TugasTantangan-IF2211_TSP_13523028
    ```
 3. Start the application using Docker Compose:
    ```bash
    chmod +x run | ./run   # For Linux
  
    run.bat # For Windows
    ```
 ---
## Screenshot Program
- Input
  
![image](https://github.com/user-attachments/assets/237a2175-aa7b-49a5-8927-1bf0b48005dc)
- Output

![image](https://github.com/user-attachments/assets/7edcc431-3365-443e-b60c-e988a304dd98)


 
 <!-- CONTRIBUTOR -->
## Author
**Muhammad Aditya Rahmadeni - 13523028**
