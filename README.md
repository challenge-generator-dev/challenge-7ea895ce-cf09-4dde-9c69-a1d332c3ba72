# Implementación de API REST de gestión de tareas en Rust

El equipo de desarrollo necesita una API REST para gestionar tareas en un proyecto de backend utilizando Rust y Actix Web. La API debe soportar la creación, listado y actualización de tareas. Cada tarea tendrá un identificador único, un título, una descripción y un estado (pendiente, en progreso, completada). El sistema debe asegurar que los títulos de las tareas no sean duplicados y que las descripciones tengan al menos 10 caracteres. En caso de error en la validación, la API debe retornar un mensaje descriptivo.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | api-rest-tareas-rust |
| **Nivel** | senior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y entregará cada archivo con su ruta.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y entregará cada archivo con su ruta. Guárdalos en la carpeta donde quieras trabajar, respetando las rutas.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no entrega todos los archivos al terminar el análisis, escríbele: "entrégame todos los archivos con su ruta".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición y creación de endpoints básicos

**Objetivo:** Implementar los endpoints REST para crear y listar tareas.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Define el modelo de datos para una tarea, incluyendo los campos requeridos.
- Implementa el endpoint para crear una nueva tarea, asegurándote de que el título sea único y la descripción tenga al menos 10 caracteres.
- Implementa el endpoint para listar todas las tareas existentes.

**Entregable:** Endpoints funcionales para crear y listar tareas, con validaciones básicas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo representar los estados de las tareas en tu modelo de datos.
- Piensa en cómo manejarías los errores de validación en los endpoints.

</details>

### Fase 2: Actualización de tareas y manejo de errores

**Objetivo:** Implementar el endpoint para actualizar tareas y mejorar el manejo de errores.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Implementa el endpoint para actualizar una tarea existente, permitiendo cambiar su título, descripción y estado.
- Mejora el manejo de errores para proporcionar mensajes descriptivos en caso de fallas de validación o intentos de actualización de tareas inexistentes.

**Entregable:** Endpoint funcional para actualizar tareas, con mejoras en el manejo de errores.

<details>
<summary>Pistas de conocimiento</summary>

- Reflexiona sobre cómo podrías estructurar los mensajes de error para que sean útiles para el usuario final.
- Considera las posibles formas en que una tarea podría no existir y cómo manejarías esos casos.

</details>

### Fase 3: Refactorización y optimización

**Objetivo:** Refactorizar el código para mejorar su legibilidad y rendimiento, y agregar funcionalidades adicionales si es necesario.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Revisa el código implementado en las fases anteriores y realiza las refactorizaciones necesarias para mejorar su legibilidad y estructura.
- Identifica posibles optimizaciones para mejorar el rendimiento de la API, especialmente en la creación y actualización de tareas.
- Agrega cualquier funcionalidad adicional que consideres necesaria para completar la API, como la eliminación de tareas.

**Entregable:** Código refactorizado y optimizado, con posibles funcionalidades adicionales implementadas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera patrones de diseño que podrían aplicarse para mejorar la estructura del código.
- Piensa en cómo podrías optimizar las consultas a la base de datos para mejorar el rendimiento.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una tarea en el contexto de esta API y cuáles son sus atributos principales?
- **paraQueSirve**: ¿Para qué sirve cada endpoint implementado en esta API y cómo contribuyen a la gestión de tareas?
- **comoSeUsa**: ¿Cómo se usa la API para crear, listar y actualizar tareas, y qué validaciones se aplican en cada caso?
- **erroresComunes**: ¿Cuáles son los errores comunes que pueden ocurrir al usar esta API y cómo se manejan?
- **queDecisionesImplica**: ¿Qué decisiones de diseño implica la implementación de esta API y cómo afectan su rendimiento y legibilidad?

## Criterios de Evaluacion

- Implementación correcta de los endpoints para crear y listar tareas con las validaciones requeridas.
- Implementación correcta del endpoint para actualizar tareas y mejora en el manejo de errores.
- Refactorización y optimización del código para mejorar su legibilidad y rendimiento.
- Adición de funcionalidades adicionales si es necesario para completar la API.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT) si no tenes un agente en el IDE. Declara lo mismo que AGENTS.md; los archivos que te devuelva el chat los guardas en esta carpeta respetando las rutas.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
cargo check
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
