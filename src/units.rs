//! QUDT is "Quantities, Units, Dimensions and Data Types in OWL and XML"
//!
//! This module contains the QUDT unit definitions. using RDF/XML syntax to load the QUDT ontology
//! Provides a set of unit definitions and conversion functions for various physical quantities
//! 
//! The QUDT ontology is a standard for representing units of measurement and physical quantities
//! in RDF/XML format. It provides a comprehensive set of unit definitions and conversion functions
//! for various physical quantities, including length, mass, time, temperature, and more.
//!
//! The QUDT ontology is widely used in scientific and engineering applications, and is supported by
//! many software tools and libraries. It is an important resource for anyone working with physical quantities
//! and units of measurement, and provides a standardized way to represent and manipulate these concepts
//! in software applications.
//!
//! The QUDT ontology is available at: http://www.qudt.org/
//!
//! The QUDT ontology is licensed under the Apache License, Version 2.0. See the LICENSE file for details.


//! allocate ontology path to the QUDT ontology file
// fn find_ontology_path() -> Option<String> {
//     let mut path = std::env::current_dir().ok()?;
//     path.push("src/units/qudt.owl");
//     if path.exists() {
//         Some(path.to_str()?.to_string())
//     } else {
//         None
//     }
// }

