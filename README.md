# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?
- Tidak, kita tidak perlu menggunakan interface dalam kasus ini, karena satu Model struct saja sudah mencukupi. Setiap subscriber memiliki fungsi yang sama, yaitu menerima notifikasi melalui HTTP berdasarkan product_type yang mereka subscribe.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?
- Menggunakan `DashMap` lebih disarankan dalam kasus ini karena memungkinkan penyimpanan data dengan key yang tetap unik, seperti `id` pada `Program` dan `url` pada `Subscriber`.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?
- Ya, penggunaan `DashMap` tetap diperlukan meskipun telah menerapkan pola desain Singleton. Singleton hanya memastikan bahwa data tersedia secara global dalam satu instance, tetapi tidak menjamin keamanan akses ketika digunakan secara bersamaan oleh banyak thread. Oleh karena itu, `DashMap` berperan penting dalam memastikan akses ke data `SUBSCRIBERS` tetap aman dalam lingkungan yang mendukung eksekusi paralel.

#### Reflection Publisher-2
1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?
- Pemisahan "Service" dan "Repository" dari "Model" dalam pengembangan perangkat lunak didasarkan pada prinsip desain seperti Single Responsibility Principle (SRP) dan Separation of Concerns (SoC) untuk meningkatkan modularitas, maintainability, dan skalabilitas kode. "Repository" bertanggung jawab khusus untuk interaksi dengan penyimpanan data (seperti database), sementara "Service" menangani logika bisnis yang kompleks, memisahkan kedua tanggung jawab ini dari "Model" yang sebaiknya hanya merepresentasikan struktur data dan validasi dasar. Dengan demikian, perubahan dalam lapisan penyimpanan atau logika bisnis tidak saling memengaruhi, memudahkan pengujian (testability), serta memungkinkan reuse komponen yang lebih fleksibel.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?
- Jika semua fungsi ditangani langsung oleh Model tanpa pemisahan, kompleksitas kode akan meningkat seiring bertambahnya fitur. Ketergantungan antar Model juga menjadi lebih besar, sehingga perubahan kecil pada satu Model dapat mempengaruhi keseluruhan sistem. Selain itu, akan sulit menghindari duplikasi kode karena setiap Model harus menangani logika bisnisnya sendiri tanpa abstraksi yang jelas. Hal ini bertentangan dengan prinsip DRY, membuat refactoring lebih rumit, serta meningkatkan kemungkinan terjadinya kesalahan saat melakukan perubahan.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.
- Postman sangat membantu dalam pengujian API karena menyediakan antarmuka mudah untuk mengirim request (GET, POST, dll.) dan memeriksa respons, seperti yang pernah saya gunakan dalam mata kuliah Pemrograman Berbasis Platform (PBP) untuk menguji API endpoints dengan efisien dan menampilkan respons data secara jelas. Fitur yang berguna untuk proyek grup atau masa depan antara lain: Collections yang mempermudah pengelompokan request API terkait agar lebih terorganisir, Environment Variables untuk mengelola konfigurasi, Automated Testing dengan skrip pengujian otomatis guna memastikan setiap endpoint memberikan respons sesuai harapan, serta Mock Servers untuk simulasi API sebelum backend siap.

#### Reflection Publisher-3
1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?
- Untuk tutorial ini, kita menggunakan Push Model dalam Observer Pattern. Model ini memungkinkan publisher (sistem) untuk secara langsung mengirimkan notifikasi ke subscribers begitu terjadi perubahan pada status produk. Pendekatan ini tercermin dalam implementasi `NotificationService::notify()`, di mana sistem segera meneruskan notifikasi ke subscribers dengan memanggil `subscriber_clone.update(payload_clone)`, tanpa menunggu permintaan dari mereka.

2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)
- Jika kita menggunakan Pull Model dalam kasus ini, salah satu keuntungannya adalah beban publisher menjadi lebih ringan karena tidak harus mengirim notifikasi ke setiap subscriber secara langsung. Sebaliknya, subscribers bisa mengambil data sesuai kebutuhan mereka. Namun, kelemahannya adalah subscribers harus secara rutin melakukan polling ke publisher untuk mengecek perubahan status produk. Pendekatan ini bisa menjadi kurang efisien karena membutuhkan pengecekan terus-menerus, tidak seperti Push Model yang langsung mengirimkan notifikasi begitu terjadi perubahan.

3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.
- Jika proses notifikasi dijalankan tanpa multithreading, aplikasi akan kesulitan menangani banyak subscribers karena setiap pembaruan harus dikirim satu per satu dalam satu thread. Akibatnya, proses pengiriman notifikasi bisa menjadi lambat dan menghambat kinerja sistem. Dengan menerapkan multithreading menggunakan `thread::spawn()`, notifikasi dapat dikirim secara paralel, sehingga eksekusi utama aplikasi tidak terblokir dan sistem dapat berjalan lebih efisien.
