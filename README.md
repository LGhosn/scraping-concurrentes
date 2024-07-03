# TP1 para la materia Programación Concurrente - FIUBA

## Introducción
Stack Exchange publica periódicamente el dump de todas las preguntas y respuestas realizadas en los sitios de su red.

Queremos aprovechar esto para realizar un análisis de los temas que se hablan en los sitios. Para ello utilizaremos los tags que cada pregunta tiene asociada.

## Uso
``` bash
cargo run --release <num_hilos>
```

## Objetivo
Implementar una aplicación en Rust para procesamiento de información, aprovechando las ventajas del modelo Fork-Join, utilizando el dataset https://huggingface.co/datasets/flax-sentence-embeddings/stackexchange_title_body_jsonl

## Requerimientos
- La aplicación debe procesar todos los archivos `.jsonl` en el subdirectorio `data`. Puede utilizar el comando `download_data.sh` para descargar el dataset y descomprimirlo automáticamente.
- Debe recibir un único parámetro entero por linea de comandos indicando la cantidad de worker threads con la cual procesar la información
- Debe presentar un resultado final de procesamiento en JSON por consola con el siguiente formato
``` json
{
    "padron": <número de padron del alumno>,
    "sites": {
        "site1": {
            "questions": <cantidad total de preguntas para ese sitio>,
            "words": <cantidad total de palabras para ese sitio>,
            "tags": {
                "tag1": {
                    "questions": <cantidad total de preguntas para ese tag para ese sitio>,
                    "words": <cantidad total palabras para ese tag para ese sitio>,
                },
                ...
                "tagN": {

                },
            }
            "chatty_tags": [
                "tag1", "tag2", ... // los 10 tags con mayor relación words/questions para ese sitio
            ]
        },
        ...
        "siteN" : {
            ...
        }
    },
    "tags": {
        "tag1": {
            "questions": <cantidad total de preguntas para ese tag para todos los sitios>,
            "words": <cantidad total palabras para ese tag para todos los sitios>,
        },
        ...
        "tagN": {

        },
    },
    "totals": {
        "chatty_sites": [
            "site1", "site2", ... // los 10 sitios con mayor relación words/questions
        ],
        "chatty_tags": [
            "tag1", "tag2", ... // los 10 tags con mayor relación words/questions entre todos los sitios.
        ]
    }
}
```
## Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución de los ejercicios:

- El proyecto deberá ser desarrollado en lenguaje Rust, usando las herramientas de la biblioteca estándar.
- El archivo Cargo.toml se debe encontrar en la raíz del repositorio, para poder ejecutar correctamente los tests automatizados
- Se deberán utilizar las herramientas de concurrencia correspondientes al modelo forkjoin
- No se permite utilizar crates externos, salvo los explícitamente mencionados en este enunciado, en los ejemplos de la materia, o autorizados expresamente por los profesores. Para el - - - procesamiento de JSON se puede utilizar el crate serde_json.
- El código fuente debe compilarse en la última versión stable del compilador y no se permite utilizar bloques unsafe.
- El código deberá funcionar en ambiente Unix / Linux.
- El programa deberá ejecutarse en la línea de comandos.
- La compilación no debe arrojar warnings del compilador, ni del linter clippy.
- Las funciones y los tipos de datos (struct) deben estar documentadas siguiendo el estándar de cargo doc.
- El código debe formatearse utilizando cargo fmt.
- Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.

## Entrega
La resolución del presente proyecto es individual.

La entrega del proyecto se realizará mediante Github Classroom. Cada estudiante tendrá un repositorio disponible para hacer diferentes commits con el objetivo de resolver el problema propuesto. Se recomienda iniciar tempranamente y hacer commits pequeños agreguen funcionalidad incrementalmente. Se podrán hacer commit hasta el día de la entrega a las 19 hs Arg, luego el sistema automáticamente quitará el acceso de escritura.

## Evaluación
Principios teóricos y corrección de bugs
La evaluación se realizará sobre Github, pudiendo el profesor hacer comentarios en el repositorio y solicitar cambios o mejoras cuando lo encuentre oportuno, especialmente debido al uso incorrecto de herramientas de concurrencia.

## Casos de prueba
Se someterá a la aplicación a diferentes casos de prueba que validen la correcta aplicación de las herramientas de concurrencia.

## Organización del código
El código debe organizarse respetando los criterios de buen diseño y en particular aprovechando las herramientas recomendadas por Rust. Se prohibe el uso de bloques unsafe.

## Tests automatizados
La presencia de tests automatizados que prueben diferentes casos, en especial sobre el uso de las herramientas de concurrencia es un plus.

## Presentación en término
El trabajo deberá entregarse para la fecha estipulada. La presentación fuera de término sin coordinación con antelación con el profesor influye negativamente en la nota final.