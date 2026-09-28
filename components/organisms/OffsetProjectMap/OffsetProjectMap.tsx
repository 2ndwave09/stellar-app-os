'use client';

import { useEffect, useMemo, type JSX } from 'react';
import { MapContainer, TileLayer, CircleMarker, Tooltip } from 'react-leaflet';
import type { CarbonProject, ProjectType } from '@/lib/types/carbon';
import 'leaflet/dist/leaflet.css';

// ─── Types ────────────────────────────────────────────────────────────────────

export interface OffsetProjectMapFilters {
  /** Project types to show; empty array = all */
  types: ProjectType[];
  /** Regions (location strings) to show; empty array = all */
  regions: string[];
  /** When true, hides out-of-stock projects */
  availableOnly: boolean;
}

export interface OffsetProjectMapProps {
  projects: CarbonProject[];
  filters: OffsetProjectMapFilters;
  className?: string;
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

const TYPE_COLORS: Record<ProjectType, { fill: string; stroke: string }> = {
  Reforestation: { fill: '#00B36B', stroke: '#059669' },
  'Renewable Energy': { fill: '#14B6E7', stroke: '#0284c7' },
  'Mangrove Restoration': { fill: '#3E1BDB', stroke: '#4f46e5' },
  'Sustainable Agriculture': { fill: '#f59e0b', stroke: '#d97706' },
  Other: { fill: '#94a3b8', stroke: '#64748b' },
};

function markerColor(type: ProjectType, isOutOfStock: boolean) {
  if (isOutOfStock) return { fill: '#94a3b8', stroke: '#64748b' };
  return TYPE_COLORS[type] ?? TYPE_COLORS.Other;
}

// ─── Component ────────────────────────────────────────────────────────────────

export function OffsetProjectMap({
  projects,
  filters,
  className,
}: OffsetProjectMapProps): JSX.Element {
  // Fix default Leaflet icon URLs in Next.js
  useEffect(() => {
    void import('leaflet').then((L) => {
      // @ts-expect-error — Leaflet internal
      delete L.Icon.Default.prototype._getIconUrl;
      L.Icon.Default.mergeOptions({
        iconRetinaUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon-2x.png',
        iconUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon.png',
        shadowUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-shadow.png',
      });
    });
  }, []);

  const filtered = useMemo(() => {
    return projects.filter((p) => {
      if (filters.types.length > 0 && !filters.types.includes(p.type)) return false;
      if (filters.regions.length > 0 && !filters.regions.includes(p.location)) return false;
      if (filters.availableOnly && p.isOutOfStock) return false;
      return true;
    });
  }, [projects, filters]);

  return (
    <MapContainer
      center={[10, 15]}
      zoom={2}
      scrollWheelZoom={false}
      className={`h-full w-full rounded-xl ${className ?? ''}`}
      aria-label="Carbon offset projects map"
    >
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />

      {filtered.map((project) => {
        const { fill, stroke } = markerColor(project.type, project.isOutOfStock);
        const { latitude: lat, longitude: lng } = project.coordinates;

        return (
          <CircleMarker
            key={project.id}
            center={[lat, lng]}
            radius={10}
            pathOptions={{
              color: stroke,
              fillColor: fill,
              fillOpacity: project.isOutOfStock ? 0.35 : 0.8,
              weight: 2,
            }}
          >
            <Tooltip>
              <div style={{ minWidth: 180 }}>
                <strong>{project.name}</strong>
                <br />
                <span>📍 {project.location}</span>
                <br />
                <span>🌿 {project.type}</span>
                <br />
                <span>
                  💰 ${project.pricePerTon.toFixed(2)} / tonne · ✅ {project.verificationStatus}
                </span>
                <br />
                {project.isOutOfStock ? (
                  <span style={{ color: '#ef4444', fontWeight: 600 }}>Out of stock</span>
                ) : (
                  <span style={{ color: '#00B36B', fontWeight: 600 }}>
                    {project.availableSupply.toLocaleString()} t available
                  </span>
                )}
                {project.coBenefits.length > 0 && (
                  <>
                    <br />
                    <span>🌍 {project.coBenefits.join(', ')}</span>
                  </>
                )}
              </div>
            </Tooltip>
          </CircleMarker>
        );
      })}
    </MapContainer>
  );
}
