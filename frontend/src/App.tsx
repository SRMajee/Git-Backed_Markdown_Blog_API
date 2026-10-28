import { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';

interface PostSummary {
  slug: string;
  title: string;
}

interface PostDetail {
  slug: string;
  title: string;
  markdown: string;
  html: string;
}

export default function App() {
  const [posts, setPosts] = useState<PostSummary[]>([]);
  const [selectedSlug, setSelectedSlug] = useState<string | null>(null);
  const [activePost, setActivePost] = useState<PostDetail | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const API_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000/api';

  // Fetch all posts on initial mount
  useEffect(() => {
    fetch(`${API_URL}/posts`)
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP error! status: ${res.status}`);
        return res.json();
      })
      .then((data: PostSummary[]) => {
        setPosts(data);
        if (data.length > 0) {
          setSelectedSlug(data[0].slug);
        }
      })
      .catch((err) => setError(err.message));
  }, []);

  // Fetch post details when selected slug changes
  useEffect(() => {
    if (!selectedSlug) return;

    setLoading(true);
    fetch(`${API_URL}/posts/${selectedSlug}`)
      .then((res) => {
        if (!res.ok) throw new Error(`Failed to load post: ${res.statusText}`);
        return res.json();
      })
      .then((data: PostDetail) => {
        setActivePost(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, [selectedSlug]);

  return (
    <div style={{ display: 'flex', minHeight: '100vh', fontFamily: 'system-ui, -apple-system, sans-serif', color: '#0f172a' }}>
      {/* Sidebar: Post List */}
      <aside style={{ width: '280px', borderRight: '1px solid #e2e8f0', padding: '1.5rem', background: '#f8fafc' }}>
        <h2 style={{ fontSize: '1.25rem', fontWeight: 700, marginBottom: '1.25rem', color: '#1e293b' }}>
          🦀 Git Markdown Blog
        </h2>
        {error && (
          <div style={{ background: '#fef2f2', border: '1px solid #fecaca', padding: '0.75rem', borderRadius: '6px', color: '#dc2626', fontSize: '0.875rem', marginBottom: '1rem' }}>
            {error}
          </div>
        )}
        <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
          {posts.map((post) => (
            <li key={post.slug} style={{ marginBottom: '0.5rem' }}>
              <button
                onClick={() => setSelectedSlug(post.slug)}
                style={{
                  width: '100%',
                  textAlign: 'left',
                  padding: '0.65rem 0.85rem',
                  borderRadius: '6px',
                  border: 'none',
                  background: selectedSlug === post.slug ? '#2563eb' : 'transparent',
                  color: selectedSlug === post.slug ? '#ffffff' : '#334155',
                  fontWeight: selectedSlug === post.slug ? 600 : 400,
                  cursor: 'pointer',
                  textTransform: 'capitalize',
                  transition: 'background 0.15s ease',
                }}
              >
                {post.title}
              </button>
            </li>
          ))}
        </ul>
      </aside>

      {/* Main Content Area */}
      <main style={{ flex: 1, padding: '2.5rem 4rem', maxWidth: '850px' }}>
        {loading && <p style={{ color: '#64748b' }}>Loading article...</p>}
        {!loading && activePost && (
          <article style={{ lineHeight: 1.7, fontSize: '1.05rem' }}>
            <ReactMarkdown>{activePost.markdown}</ReactMarkdown>
          </article>
        )}
        {!loading && !activePost && !error && (
          <p style={{ color: '#64748b' }}>Select a post from the sidebar to view it.</p>
        )}
      </main>
    </div>
  );
}
