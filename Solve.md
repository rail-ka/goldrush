# Решение

1. Получаем площади с сундуками, допустим оптимальным будет поиск 100х100.
2. Если на заданном поле есть сундуки - отправляем в обработку поле.
3. В самом начале лицензий нет поэтому за время обработки карты мы еще получаем бесплатные лицензии. Сколько успеем, потом будет покупать платные лиценции.
4. Когда поле попадает в обработку, нужно узнать есть ли у нас лицензии. Если есть - копаем, если нет - покупаем или берем бесплатные если нет денег.
5. Какую сумму указывать за лицензию?
6. Берем лицензию и смотрим осталось ли у нее разрешения на копку. Если нет - берем новую лицензию. Если есть - начинаем копать.
7. Нужно ли еще раз проверять площадь на наличие сундуков? Наверное да, смотреть на вероятность одного клада в обрабатываемом поле. Если вероятность слишком маленькая, то еще раз проверять поле на наличие сунуков с уменьшенной площадью. Нужно ли проверять одну точку?
8. Копать. Если в данной точке есть сундуки - копать, пока не найдем то количество сундуков, которое определено на площади. Можно ли копать одновременно в нескольких точках на площади? Если нашли все сундуки - остановить копание во всех точках. Если дошли до -10 этажа - в данной точке копать прекращаем.
9. Меняем сундуки на деньги. Когда меняем? Сразу или потом?
10. Когда делаются по очереди Explore много раз, сервер захлебывается, нужно видимо давать интервал другими запросами? Хотя среднее значение только в 2 раза больше минимального, значит нужно сбрасывать долгие подключения (более 3-5мс?)

10 минут = 60 * 10 = 600 секунд = 600_000 миллисекунд

1/2 area (1/4 for every cpu)
cpu: 2, for every:
size_x: 35 end: 447_164
areas_count: 87500 areas_without_gold: 20874, profit: 4 (24%)
explore min: 1516 max: 23019 avg: 5109
error_code: 1000, count: 2489
error_code: 1002, count: 33

full area (1/2 for every cpu)
cpu: 2, for every:
size_x: 70 end: 532_182
areas_count: 87500 areas_without_gold: 4832, profit: 18 (6%)
explore min: 1800 max: 20501 avg: 6080
error_code: 1000, count: 6476
error_code: 1002, count: 32

1/2 area
cpu: 1
size_x: 50
end: 376_157
areas_count: 122_500 areas_without_gold: 16121, profit: 7,4 (14%)
explore min: 1632 max: 23707 avg: 3069
amounts min: 1 max: 11 avg: 2
error_code: 1000, count: 2214
error_code: 1002, count: 94

логи 15k или 150 строк

у меня в графане почему то появился третий дашборд?

чтобы просто догнать участников на обмен монет нужно 200 секунд, что является 1/3 от общего времени:
47 монет в сундуке максимум, 900к монет - это примерно 19к сундуков, на один сундук 10мс = 190 секунд.

исследовать моэно не все поле, так как времени все равно не хватит. Поэтому исследуем например 1/3 поля пока, увеличивая size_x. если брать 1/20 поля, то теоретически можно получить 1млн монет. если 1/10 - то 2млн... для 5.7млн монет нужно исследовать 1/4 поля.

разделять на cpu похоже не имеет смысла, так как время все равно увеличивается вдвое... и ошибок больше и среднее время запроса увеличивается...

возможно нужна приоритетная очередь для обработки участков с бОльшим количеством сундуков первыми...
есть залежи сундуков...