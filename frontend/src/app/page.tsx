export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800">
      <div className="container mx-auto px-4 py-16">
        <div className="text-center">
          <h1 className="text-5xl font-bold text-white mb-4">
            MTG AI Suite
          </h1>
          <p className="text-xl text-gray-300 mb-8">
            Magic: The Gathering AI-powered toolkit
          </p>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mt-12">
            <FeatureCard
              title="Card Database"
              description="Search and explore the complete MTG card database with semantic search"
              icon="🃏"
            />
            <FeatureCard
              title="Deck Builder"
              description="AI-powered deck building with archetype analysis and suggestions"
              icon="📚"
            />
            <FeatureCard
              title="Game Tracker"
              description="Real-time game state tracking and play history"
              icon="📊"
            />
            <FeatureCard
              title="Play Advisor"
              description="Get intelligent suggestions for optimal plays"
              icon="🧠"
            />
          </div>
        </div>
      </div>
    </main>
  )
}

function FeatureCard({ 
  title, 
  description, 
  icon 
}: { 
  title: string
  description: string
  icon: string 
}) {
  return (
    <div className="bg-gray-800 rounded-lg p-6 border border-gray-700 hover:border-blue-500 transition-colors">
      <div className="text-4xl mb-4">{icon}</div>
      <h3 className="text-xl font-semibold text-white mb-2">{title}</h3>
      <p className="text-gray-400">{description}</p>
    </div>
  )
}
