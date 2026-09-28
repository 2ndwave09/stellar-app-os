import jsPDF from 'jspdf';

export interface CorporateBranding {
  companyName: string;
  logoUrl?: string;
  primaryColor?: string;
}

export interface EsgReportData {
  companyName: string;
  totalTrees: number;
  totalCo2Offset: number;
  projectsSupported: string[];
  period: string;
  reportId: string;
  offsets?: Array<{
    projectName: string;
    creditType: string;
    tonnesCo2e: number;
    verification: string;
  }>;
}

export function generateEsgReport(data: EsgReportData): void {
  const doc = new jsPDF();
  const pageWidth = doc.internal.pageSize.getWidth();

  // Header
  doc.setFillColor(13, 11, 33); // Stellar Navy
  doc.rect(0, 0, pageWidth, 40, 'F');

  doc.setTextColor(255, 255, 255);
  doc.setFontSize(22);
  doc.text('ESG IMPACT REPORT', pageWidth / 2, 25, { align: 'center' });

  // Content
  doc.setTextColor(0, 0, 0);
  doc.setFontSize(12);
  let y = 55;

  doc.setFont('helvetica', 'bold');
  doc.text('Organization:', 20, y);
  doc.setFont('helvetica', 'normal');
  doc.text(data.companyName, 60, y);

  y += 10;
  doc.setFont('helvetica', 'bold');
  doc.text('Reporting Period:', 20, y);
  doc.setFont('helvetica', 'normal');
  doc.text(data.period, 60, y);

  y += 20;
  doc.setFontSize(16);
  doc.setTextColor(20, 182, 231); // Stellar Blue
  doc.text('Cumulative Impact Metrics', 20, y);

  y += 15;
  doc.setFillColor(241, 245, 249);
  doc.roundedRect(20, y, pageWidth - 40, 30, 3, 3, 'F');

  doc.setTextColor(0, 0, 0);
  doc.setFontSize(12);
  doc.text('Total Trees Planted:', 30, y + 12);
  doc.setFontSize(14);
  doc.text(data.totalTrees.toLocaleString(), 120, y + 12);

  doc.setFontSize(12);
  doc.text('Total CO2 Sequestered:', 30, y + 22);
  doc.setFontSize(14);
  doc.text(`${data.totalCo2Offset.toLocaleString()} tCO2e`, 120, y + 22);

  y += 45;
  doc.setFontSize(16);
  doc.setTextColor(20, 182, 231);
  doc.text('Supported Restoration Projects', 20, y);

  y += 10;
  doc.setFontSize(10);
  doc.setTextColor(100, 116, 139);

  const pageHeight = doc.internal.pageSize.getHeight();
  const bottomMargin = 25;
  const ensureSpace = (needed: number) => {
    if (y + needed > pageHeight - bottomMargin) {
      doc.addPage();
      y = 25;
      doc.setFontSize(10);
      doc.setTextColor(100, 116, 139);
    }
  };

  if (data.offsets?.length) {
    data.offsets.forEach((line) => {
      ensureSpace(14);
      doc.setFont('helvetica', 'bold');
      doc.setTextColor(0, 0, 0);
      doc.text(`• ${line.projectName}`, 25, y);
      doc.text(`${line.tonnesCo2e.toLocaleString()} tCO2e`, pageWidth - 20, y, { align: 'right' });
      doc.setFont('helvetica', 'normal');
      doc.setTextColor(100, 116, 139);
      doc.text(`${line.creditType} · ${line.verification}`, 30, y + 5);
      y += 14;
    });
  } else {
    data.projectsSupported.forEach((project) => {
      ensureSpace(7);
      doc.text(`• ${project}`, 25, y);
      y += 7;
    });
  }

  // Footer (on every page)
  const pageCount = doc.getNumberOfPages();
  for (let page = 1; page <= pageCount; page += 1) {
    doc.setPage(page);
    doc.setFontSize(8);
    doc.setTextColor(150, 150, 150);
    doc.text(`Report ID: ${data.reportId} | Page ${page} of ${pageCount}`, pageWidth / 2, 285, {
      align: 'center',
    });
  }

  doc.save(`esg-report-${data.companyName.replace(/\s+/g, '-').toLowerCase()}.pdf`);
}
