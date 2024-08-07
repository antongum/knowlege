//_ МЬЮТЕКСЫ
// это тип данных
// который позволяет заблокировать доступ к расшариваемым данным 
// в других каналах
// в момент их исползования в нужном канале
// все горутины заблокируются пока данные не будут разлочены

//~ типы
// есть два вида мьютексов
// mutex
// RWmutex

//_ MUTEX
// блокирует данные не зависимо от операции(чтение или запись)

// создаю общий ресурс
var counter int = 0    

// определяем мьютекс
var mutex sync.Mutex        

// запускаю несколько горутин передавая мьютекс
for i := 1; i < 5; i++{
    go work(&mutex)  
}

func work (number int, ch chan bool, mutex *sync.Mutex){
    mutex.Lock()    // блокируем доступ к переменной counter
    counter++       // изменяю общий ресурс 
    mutex.Unlock()  // разблокирую для других горутин
}