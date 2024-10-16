use crate::enums::Projects;
use crate::templates;

pub fn generate_dockerfile(project: Projects) -> String {
    match project {
        /*
           TODO - Currently using test params, remove it and take user provided parameters
           TODO - Take image_name, app_name and CMD from user input or read the root file and package file
           TODO - Check for root file for none binary projects and check for root file like app, index, server, main etc
           TODO - Check for flask and django project and check for respective run CMD
           TODO - Try reading the filenames, entrypoints, package files, extensively to make user work less and prompt with their data
        */
        Projects::Nodejs => templates::node::node_dkfl("node:20-alpine", "dkfl", "/home"),
        Projects::Rust => templates::rust::rust_dkfl("rust:latest", "dkfl", "/home"),
        Projects::Go => templates::go::go_dkfl("golang:1.20-alpine", "dkfl", "/home"),
        Projects::Python => templates::python::python_dkfl("python:3.11-slim", "dkfl", "/home"),
        Projects::Ruby => templates::ruby::ruby_dkfl("ruby:3.2", "dkfl", "/home"),
        Projects::Php => templates::php::php_dkfl("php:8.2-cli", "dkfl", "/home"),
        Projects::GradleJava => templates::gradle::gradle_dkfl("gradle:7.6-jdk17", "dkfl", "/home"),
        Projects::MavenJava => {
            templates::maven::maven_dkfl("maven:3.9-eclipse-temurin-17", "dkfl", "/home")
        }

        Projects::Unknown => {
            // TODO - Give a proper warning/error message here...
            todo!()
        }
    }
}
