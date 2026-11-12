"""
LLM Engine for threat analysis using Ollama
"""

import asyncio
import logging
from typing import List, Optional, Dict, Any
from dataclasses import dataclass
from datetime import datetime

try:
    import ollama
except ImportError:
    ollama = None
    logging.warning("Ollama not installed. Install with: pip install ollama")

logger = logging.getLogger(__name__)


@dataclass
class ThreatAnalysis:
    """Result of AI threat analysis"""
    risk_score: int  # 0-100
    confidence: int  # 0-100
    explanation: str
    mitre_tactics: List[str]
    mitre_techniques: List[str]
    indicators_of_compromise: List[str]
    recommended_actions: List[str]
    false_positive_likelihood: int  # 0-100


class LLMEngine:
    """LLM engine wrapper for Ollama"""
    
    def __init__(self, model: str = "mistral:7b-instruct-q4_K_M"):
        self.model = model
        self.context_window = 8192
        self.temperature = 0.1  # Low temperature for consistency
        
        if ollama is None:
            logger.error("Ollama not available")
            self.available = False
        else:
            self.available = True
            logger.info(f"LLM Engine initialized with model: {model}")
    
    async def analyze_threat(
        self,
        event: Dict[str, Any],
        context: Optional[Dict[str, Any]] = None,
        mitre_context: Optional[str] = None
    ) -> ThreatAnalysis:
        """
        Analyze security event using LLM with RAG
        
        Args:
            event: Security event data
            context: Additional context from graph database
            mitre_context: MITRE ATT&CK context from RAG pipeline
        
        Returns:
            ThreatAnalysis with risk assessment
        """
        if not self.available:
            return self._fallback_analysis(event)
        
        # Build prompt with context
        prompt = self._build_prompt(event, context, mitre_context)
        
        try:
            # Generate analysis
            response = await self._generate(prompt)
            
            # Parse response
            analysis = self._parse_response(response, event)
            
            logger.info(f"Threat analysis complete: risk_score={analysis.risk_score}")
            return analysis
            
        except Exception as e:
            logger.error(f"LLM analysis failed: {e}")
            return self._fallback_analysis(event)
    
    def _build_prompt(
        self,
        event: Dict[str, Any],
        context: Optional[Dict[str, Any]],
        mitre_context: Optional[str]
    ) -> str:
        """Build analysis prompt with RAG context"""
        
        prompt = f"""You are a cybersecurity expert analyzing a security event.

Event Details:
- Type: {event.get('event_type', 'unknown')}
- Process: {event.get('process', {}).get('name', 'unknown')}
- User: {event.get('process', {}).get('user', 'unknown')}
- Timestamp: {event.get('timestamp', 'unknown')}

"""
        
        if mitre_context:
            prompt += f"\nMITRE ATT&CK Context:\n{mitre_context}\n"
        
        if context:
            prompt += f"\nAdditional Context:\n{context}\n"
        
        prompt += """
Analyze this event and provide:
1. Risk score (0-100)
2. Confidence level (0-100)
3. Explanation of the threat
4. Relevant MITRE ATT&CK tactics and techniques
5. Indicators of compromise
6. Recommended actions
7. False positive likelihood (0-100)

Format your response as JSON.
"""
        
        return prompt
    
    async def _generate(self, prompt: str) -> str:
        """Generate LLM response"""
        try:
            response = ollama.generate(
                model=self.model,
                prompt=prompt,
                options={
                    "temperature": self.temperature,
                    "top_p": 0.9,
                    "num_predict": 512
                }
            )
            return response['response']
        except Exception as e:
            logger.error(f"LLM generation failed: {e}")
            raise
    
    def _parse_response(self, response: str, event: Dict[str, Any]) -> ThreatAnalysis:
        """Parse LLM response into ThreatAnalysis"""
        # TODO: Implement proper JSON parsing
        # For now, return a basic analysis
        
        return ThreatAnalysis(
            risk_score=50,
            confidence=70,
            explanation=f"Analysis of {event.get('event_type', 'unknown')} event",
            mitre_tactics=["Initial Access"],
            mitre_techniques=["T1190"],
            indicators_of_compromise=[],
            recommended_actions=["Monitor", "Investigate"],
            false_positive_likelihood=30
        )
    
    def _fallback_analysis(self, event: Dict[str, Any]) -> ThreatAnalysis:
        """Fallback analysis when LLM is unavailable"""
        logger.warning("Using fallback analysis (LLM unavailable)")
        
        # Simple rule-based analysis
        risk_score = 30
        
        process_name = event.get('process', {}).get('name', '')
        if 'suspicious' in process_name.lower():
            risk_score = 70
        if 'malware' in process_name.lower():
            risk_score = 90
        
        return ThreatAnalysis(
            risk_score=risk_score,
            confidence=50,
            explanation=f"Fallback analysis: {event.get('event_type', 'unknown')} event detected",
            mitre_tactics=[],
            mitre_techniques=[],
            indicators_of_compromise=[],
            recommended_actions=["Review manually"],
            false_positive_likelihood=50
        )
    
    async def chat(self, message: str, conversation_history: List[Dict[str, str]] = None) -> str:
        """
        Chat interface for conversational queries
        
        Args:
            message: User message
            conversation_history: Previous conversation context
        
        Returns:
            AI response
        """
        if not self.available:
            return "LLM engine not available. Please install Ollama and the required model."
        
        # Build conversation context
        prompt = self._build_chat_prompt(message, conversation_history)
        
        try:
            response = await self._generate(prompt)
            return response
        except Exception as e:
            logger.error(f"Chat failed: {e}")
            return f"Error: {str(e)}"
    
    def _build_chat_prompt(self, message: str, history: Optional[List[Dict[str, str]]]) -> str:
        """Build chat prompt with conversation history"""
        prompt = "You are a cybersecurity AI assistant. Answer questions about security threats.\n\n"
        
        if history:
            for msg in history[-5:]:  # Last 5 messages
                role = msg.get('role', 'user')
                content = msg.get('content', '')
                prompt += f"{role}: {content}\n"
        
        prompt += f"user: {message}\nassistant: "
        
        return prompt


# Singleton instance
_llm_engine: Optional[LLMEngine] = None


def get_llm_engine() -> LLMEngine:
    """Get or create LLM engine instance"""
    global _llm_engine
    if _llm_engine is None:
        _llm_engine = LLMEngine()
    return _llm_engine


if __name__ == "__main__":
    # Test the LLM engine
    async def test():
        engine = LLMEngine()
        
        test_event = {
            "event_type": "process_execution",
            "process": {
                "name": "suspicious_binary",
                "user": "www-data"
            },
            "timestamp": datetime.now().isoformat()
        }
        
        analysis = await engine.analyze_threat(test_event)
        print(f"Risk Score: {analysis.risk_score}")
        print(f"Explanation: {analysis.explanation}")
    
    asyncio.run(test())
