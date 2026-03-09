pipeline {
    agent any

    options {
        timestamps()
        disableConcurrentBuilds()
        timeout(time: 60, unit: 'MINUTES')
    }

    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        CARGO_TARGET_DIR = "${WORKSPACE}/target"
    }

    stages {
        stage('Format Check') {
            steps {
                cargo fmt --all -- --check
            }
        }

        stage('Clippy') {
            steps {
                sh 'cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --all-features'
            }
        }

        stage('Release Build') {
            steps {
                sh 'cargo build --release --all-targets'
            }
        }

        stage('Deployment') {
            when {
                branch 'main'
            }
            steps {
                sh 'echo deployed'
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'target/**/deps/*', allowEmptyArchive: true
            echo "Build finished for ${env.JOB_NAME} #${env.BUILD_NUMBER}."
        }
        success {
            echo 'Pipeline succeeded.'
        }
        failure {
            echo 'Pipeline failed. Check stage logs above for details.'
        }
    }
}
