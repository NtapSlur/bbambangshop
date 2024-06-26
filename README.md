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

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

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
1. Menurut saya, penggunaan interface diperlukan tergantung pada design pattern dari Observernya. Jika Observer yang digunakan memiliki banyak jenis, maka kita perlu membuat interface untuk Observer. Jika Observer yang digunakan hanya satu, maka tidak diperlukan interface.

1. Penggunaan DashMap pada kasus bambangshop ini agar dapat memetakan tiap jenis produk ke tiap subcriber yang memerlukannya. Hal ini lebih efisien jika dibandingkan dengan menggunakan Vec, di mana pemetaan menggunakan DashMap memakan space yang linear (O(N)). Jika ingin memetakan dengan menggunakan Vec, perlu digunakan Vec dua dimensi (matriks) untuk memetakan antara produk dengan subscriber, yang di mana memakan space yang lebih besar (O(N^2))

1. Menurut saya, DashMap diperlukan tipe data yang mirip dengan HashMap tetapi lebih cocok untuk multithreading. Hal ini sejalan dengan bambangshop yang menerapkan multithreading. Untuk singleton, menurut saya masih cocok untuk diimplementasikan agar dapat memastikan jika DashMap untuk suatu subscriber terhadap produk berada hanya pada satu dashmap

#### Reflection Publisher-2
1. Kita perlu untuk memisahkan service dengan repository agar dapat memenuhi prinsip SOLID yaitu single responsibility, di mana service berfungsi untuk mengolah data yang didapat dari repository dan repository berfungsi untuk mengakses data dari database. Selain itu, pemisahan repository dan service juga berguna untuk mengmaintain kode.

1. Jika kita menggunakan model saja, maka perubahan yang terjadi sekecil apapun dapat menyebabkan banyak perubahan ke kode kode lain, sehingga tidak efisien untuk code maintainability.

1. Postman membantu saya dalam menguji response yang dihasilkan oleh website ketika menerima sebuah request, terutama untuk TK yang di mana API berperan sangat penting karena API menjadi perantara untuk mengakses data. Oleh karena itu, diperlukan akses API yang benar sehingga diperlukannya pengujian terhadap response API.

#### Reflection Publisher-3
1. Kita menggunakan push model yang di mana dapat dilihat bahwa service akan memanggil method yang mengiterasi semua subscriber untuk mendapatkan update terbaru

1. Jika kita menggunakan pull model, nantinya tiap subscriber perlu menentukan perubahan data yang terjadi apakah relevan untuk mereka. Tetapi, observer akan lebih bebas untuk menentukan data yang akan diambil.

1. Jika kita tidak menggunakan multithreading, pada saat NotificationService perlu memberikan notifikasi kepada tiap subscriber, maka akan terjadi process notifikasi yang panjang sekali berdasarkan jumlah subscriber yang ada sehingga akan menghambat proses komputasi pada website.
