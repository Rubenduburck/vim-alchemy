use crate::error::Error;

/// Core trait for classification strategies
pub trait ClassificationStrategy: Send + Sync {
    /// Classify the input and return a classification score (0.0 to 1.0)
    fn classify(&self, input: &str) -> Result<f64, Error>;
    
    /// Get the name of this classification
    fn name(&self) -> &str;
    
    /// Get the priority of this classifier (higher = checked first)
    fn priority(&self) -> i32 {
        0
    }
}

/// Trait for classifiers that use regex patterns
pub trait RegexClassifier: ClassificationStrategy {
    /// Get the regex pattern used by this classifier
    fn pattern(&self) -> &str;
    
    /// Check if the input matches the pattern
    fn matches(&self, input: &str) -> bool;
}

/// Trait for classifiers that can provide confidence scores
pub trait ConfidenceClassifier: ClassificationStrategy {
    /// Get a confidence score for the classification (0.0 to 1.0)
    fn confidence(&self, input: &str) -> f64;
    
    /// Get the minimum confidence threshold for a positive classification
    fn threshold(&self) -> f64 {
        0.5
    }
}

/// Main classifier trait that combines multiple strategies
pub trait Classifier: Send + Sync {
    /// Classify input and return all matching classifications
    fn classify_all(&self, input: &str) -> Vec<String>;
    
    /// Get the best classification for the input
    fn classify_best(&self, input: &str) -> Option<String>;
    
    /// Add a classification strategy
    fn add_strategy(&mut self, strategy: Box<dyn ClassificationStrategy>);
    
    /// Remove a classification strategy by name
    fn remove_strategy(&mut self, name: &str) -> bool;
    
    /// Get all available classification names
    fn available_classifications(&self) -> Vec<String>;
}