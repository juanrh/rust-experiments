# rust-experiments
Experiments with Rust

## Ideas de cosas para aprender

- lists de resources de [video en YouTube](https://youtu.be/2hXNd6x9sZs): [Rust book](https://doc.rust-lang.org/stable/book/), [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/), [Rustlings](https://github.com/rust-lang/rustlings)
- Lee mis notas sobre golang para contexto sobre organizar codigo en un lenguaje no OOP
- Proyectos a contrib
   - para que rust se use en prod: aws sdk, k8s, un  api framework, gRPC 
   - del motor de vídeo juegos: https://fyrox.rs/ (antes conocido como rg3d). Pej con algo como https://github.com/FyroxEngine/Fyrox/labels/good%20first%20issue, y a largo intentando dar soporte openXR con https://mbucchia.github.io/OpenXR-Toolkit/
   - O hacer lo de Maude monitoring haciendo binding de Maude. Lo malo de eso es que no tengo comunidad. Además los de la UCM se ríen de mi con lo de que me vaya a DACIA
   - O idea de implementar workflow / step functions en k8s. O idea derivada de programar todo el sistema con un solo programa, tanto infra como cada servicio: Rust macros para capturar permisos que hacen falta etc? Se puede ver como sistema de tipos o abstract interpretation? Ver abajo en "Workflows y eventos en k8s"

### Workflows y eventos en k8s

Ejemplos de workflows / step functions he visto en mi carrera:

- transiciones de proofs en prover
- transiciones de provisión en ACS service
- aaas

Estaría bien tener una implementación open source para k8s que sea cloud agnostic

Permite escribir FSM o autómatas aun mas generales con dsl con bindings en lenguajes como Python, en plan spark, tb en Yaml: o simplemente un custom resource y útils para escribirlo
Generaliza a event based programming como cw events / evengpt bridge o lo de GCP

La idea es poder escribir un sistema completo con un solo programa Python: como con spark, haciendo captura de closures etc. Hacer system programming con un solo programa

Como de general es?
Se puede combinar con ideas de runtime monitoring para k8s? es el progtama mas analizable pq tiene una descripciin mas high level de la especificaciin? Se pueden generar solo metricas y logs y dashboards, como ocurre con spark? se pueden analizar propiedades de seguridad, o proponer permisos?

Golang parece buen lenguaje 

Hace falta consistencia fuerte. Actores o similar lleva a problema de split barain, más fácil asume DB disponible con ciertas primitivas como optimistic concurrency o transacciones

Impl de referencia en Maude para verificar implementación?

En vez de workflow comparte info explocitamente con variables del workflow, entre lenguajes 
Multiple lenguajes como corren como servicios grpc usando server less de k8s
Corre todo el sistema en local con minikube 
Primera DB es cocaroach, con spanner de sustituto en gcp 


# References
 - [Windows binaries](https://www.rust-lang.org/en-US/downloads.html#win-foot): using GNU binary, as it is more compatible with GNU SW and Linux is my target server platform
 - [Rust documentation](https://www.rust-lang.org/en-US/documentation.html)
 - IDEs
  * Sublime Text: a Rust plugin is available in the package system
  * [IntelliJ Rust plugin](https://intellij-rust.github.io/)