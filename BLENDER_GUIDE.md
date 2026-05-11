# Blender Model Creation Guide

## Быстрый старт

### 1. Создание UAZ в Blender

**Вариант A: Автоматическое создание**
1. Открой Blender
2. В меню: `Scripting` tab (вверху)
3. Вставь код из `docs/blender_create_uaz.py`
4. Нажми `Run Script` (▶)
5. Модель UAZ создастся автоматически!
6. Сохрани как `.blend` файл для дальнейшего редактирования

**Вариант B: Своя модель вручную**
1. Создай модель любым способом
2. Убедись что все объекты — Mesh
3. Выдели нужные объекты
4. `Export Selected` → текстовый файл

### 2. Экспорт модели

После создания модели в Blender:

1. Выдели объект/объекты для экспорта
2. В Scripting tab запусти `export_for_game()`
3. Файл сохранится в `assets/models/`

Или используй `blender_export.py` из командной строки:
```bash
blender model.blend --python docs/blender_export.py -- uaz
```

### 3. Использование в игре

Добавлю загрузчик моделей в код:
```rust
let mesh = ModelLoader::load("assets/models/uaz_mesh.txt")?;
```

---

## Формат файла модели

```
VERTEX_BUFFER
x y z  nx ny nz  r g b
x y z  nx ny nz  r g b
...

INDEX_BUFFER
0 1 2
3 4 5
...
```

---

## Горячие клавиши Blender

| Клавиша | Действие |
|---------|----------|
| G | Move |
| R | Rotate |
| S | Scale |
| Tab | Edit/Object mode |
| Numpad 1/3/7 | Виды |
| Ctrl+R | Loop Cut |
| Ctrl+E | Extrude |