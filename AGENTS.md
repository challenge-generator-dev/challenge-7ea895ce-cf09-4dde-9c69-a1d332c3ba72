# AGENTS.md

Instrucciones para el agente de IA que abra este repositorio (Claude Code, Cursor, Codex, Copilot, Gemini). Se cargan solas: no hay que pegar nada en ningun chat.

## Que es este repositorio

Es el codigo base de un reto de aprendizaje de Pragma: **Implementación de API REST de gestión de tareas en Rust**.

| | |
|---|---|
| Tema | api-rest-tareas-rust |
| Nivel | senior-l2 |
| Chapter | Backend |
| Especialidad | Rust + Actix Web |
| Stack | Rust / Actix Web 4.9 |
| Patron arquitectonico | capas estándar |
| Tiempo estimado | 8 horas |

## Receta del stack

Esqueleto obligatorio:

- `manifiesto de dependencias y build de Rust en la raiz (segun las convenciones idiomaticas de Rust/Actix Web)`
- `punto de entrada de la aplicacion (segun las convenciones idiomaticas de Rust/Actix Web)`
- `capa de dominio con entidades y puertos (segun las convenciones idiomaticas de Rust/Actix Web)`
- `capa de aplicacion con casos de uso (segun las convenciones idiomaticas de Rust/Actix Web)`
- `capa de infraestructura con los adaptadores y los handlers HTTP de Actix Web (segun las convenciones idiomaticas de Rust/Actix Web)`

Trampas conocidas:

- Todo el codigo, el manifiesto y los tests son de Rust/Actix Web; no mezclar otro lenguaje ni otro runtime.

Dependencias:

- actix-web 4.19.0
- anyhow 1.0.104
- async-trait 0.1.92
- dotenv 0.15.0
- serde 1.0.229
- sqlx 0.9.0
- thiserror 1.0.69
- validator 0.21.0
- log 0.4.35
- mockall 0.15.0
- uuid 1.29.2
- actix-rt 2.15.0

## Tu tarea

Dejar este proyecto en estado **verificable**: que el comando de verificacion corra sin errores. Escribi los archivos en disco, en este repositorio. No generes ZIPs ni archivos adjuntos.

En orden:

1. Corre `cargo check` y mira que falla.
2. Completa lo que falte de la lista de abajo: manifiesto de dependencias, punto de entrada, capa de interfaz y las capas del patron declarado.
3. Arregla SOLO los errores que impiden compilar o arrancar.
4. Volve a correr `cargo check` hasta que pase.
5. Pará ahí.

## Regla dura: las fases son trabajo del humano

**PROHIBIDO implementar los entregables de las fases.** El valor del reto esta en que la persona los resuelva. Tu trabajo es que tenga un proyecto que arranca; el hueco pedagogico se queda como esta.

No resuelvas nada de esto:

- **Fase 1 — Definición y creación de endpoints básicos**: Endpoints funcionales para crear y listar tareas, con validaciones básicas.
- **Fase 2 — Actualización de tareas y manejo de errores**: Endpoint funcional para actualizar tareas, con mejoras en el manejo de errores.
- **Fase 3 — Refactorización y optimización**: Código refactorizado y optimizado, con posibles funcionalidades adicionales implementadas.

Distincion operativa:

- **Arreglar** (si): import faltante, tipo que no existe, dependencia sin declarar, error de sintaxis, archivo referenciado que no existe.
- **No tocar** (no): logica de negocio incompleta, validaciones ausentes, secretos hardcodeados, APIs deprecadas que funcionan, concurrencia insegura, patrones mejorables. Eso es lo que la persona tiene que encontrar.

## Superficie de practica (NO completes)

Estos archivos SON el ejercicio de la persona. No los implementes; deja stubs. No toques la logica que el reto pide completar.

- [ ] `src/domain/infrastructure/routes.rs` — La fase "Definición y creación de endpoints básicos" se implementa aqui: este archivo es el ejercicio.
- [ ] `src/domain/infrastructure/error_handlers.rs` — La fase "Actualización de tareas y manejo de errores" se implementa aqui: este archivo es el ejercicio.

## Lo que falta y tenes que completar

No se detectaron huecos: estan los archivos declarados, el boilerplate del stack y ninguna referencia quedo colgando. Igual corre el comando de verificacion — que los archivos existan no garantiza que compilen.

### Presentes (15)

- `Cargo.toml`
- `src/main.rs`
- `src/contracts/types.rs`
- `src/domain/mod.rs`
- `src/contracts/mod.rs`
- `src/domain/config/mod.rs`
- `src/domain/models.rs`
- `src/domain/repositories.rs`
- `src/domain/application/services.rs`
- `src/domain/infrastructure/repositories.rs`
- `src/domain/infrastructure/routes.rs`
- `src/domain/infrastructure/error_handlers.rs`
- `src/domain/web/mod.rs`
- `src/domain/application/mod.rs`
- `src/domain/infrastructure/mod.rs`

### Capas del patron declarado

Son las capas que la propuesta arquitectonica plantea, marcadas segun existan hoy en el repo. Orientan donde poner el codigo nuevo; no son una condicion para compilar.

- `src/domain` (presente)
- `src/application` (ausente)
- `src/infrastructure` (ausente)
- `src/config` (ausente)

## Verificacion

```bash
cargo check
```

El comando tiene que pasar SIN implementar los archivos de la superficie de practica: solo andamiaje.

Ese comando pasando es la definicion de "terminado" para vos.

## Convenciones que tenes que respetar

- Un solo ecosistema: no declares librerias de otro lenguaje ni mezcles gestores de paquetes.
- Toda libreria que uses tiene que estar declarada en el manifiesto de dependencias.
- Todo import declarado tiene que usarse; todo tipo usado tiene que existir o venir de una dependencia declarada.
- El patron es **capas estándar**: los contratos (interfaces, puertos) los define la capa interna y los implementa la externa, nunca al revés.
- Los archivos que crees llevan implementacion real, no stubs: sin `TODO`, sin cuerpos vacios, sin `// getters y setters`.

## Contexto del candidato

Sirve para calibrar el nivel del codigo, no para resolver las fases.

- Perfil: Chapter Backend, Especialidad Desarrollador, Tecnología Rust, Senior L2
- Brecha que el reto ataca: Construir una API REST en Rust con Actix Web para gestionar tareas, Implementa endpoints REST en Rust con Actix Web para crear, listar y actualizar tareas
- Mision: Exponer la gestion de tareas por HTTP

---

*Generado por Challenge Generator — Pragma. `README.md` tiene el enunciado completo del reto para la persona. `PROMPT_MEJORA.md` es la variante para pegar en un chat, si se prefiere ese flujo.*
